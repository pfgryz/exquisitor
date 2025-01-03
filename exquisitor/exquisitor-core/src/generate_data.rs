use clap::{Parser, Subcommand, ValueEnum};
use csv::Writer as CsvWriter;
use exquisitor_core::clustering::traits::DistanceMetric;
use exquisitor_core::clustering::ALPHABET;
use exquisitor_core::io::fasta::reader::FastaReader;
use exquisitor_core::io::fasta::record::FastaRecord;
use exquisitor_core::io::fasta::writer::FastaWriter;
use exquisitor_core::io::fastq::reader::FastqReader;
use exquisitor_core::io::sequence::Sequence;
use exquisitor_core::io::traits::{Reader, Record, Writer};
use rand::prelude::{SliceRandom, StdRng};
use rand::{Rng, SeedableRng};
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, BufWriter, Result as IoResult, Write};
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(
    version = "0.1.0",
    author = "Patryk Filip Gryz",
    about = "Generate training and experiments datasets"
)]
struct Cli {
    /// Command to execute
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    /// Generate artificial samples for training / experiments
    Artificial(ArtificialCommand),
    /// Use dataset to obtain training / experiments dataset
    Dataset(DatasetCommand),
}

#[derive(Parser, Clone, Debug)]
struct Common {
    /// Seed for random generator
    #[arg(long, default_value_t = 152754665)]
    seed: u64,

    /// Size of training set
    #[arg(long, default_value_t = 0)]
    training: usize,

    /// Size of validation set
    #[arg(long, default_value_t = 0)]
    validation: usize,

    /// Size of experiments set
    #[arg(long, default_value_t = 0)]
    experiments: usize,

    /// Path to output directory
    #[arg(long, default_value = "data")]
    output: String,

    /// Name of the experiments file
    #[arg(long, default_value = None)]
    experiments_file_name: Option<String>,

    /// Length of the sequences
    #[arg(long, default_value_t = 150)]
    length: usize,

    /// Minimum similarity between anchor and positive
    #[arg(long, default_value_t = 0.8)]
    min_similarity_positive: f64,

    /// Maximum similarity between anchor and positive
    #[arg(long, default_value_t = 1.0)]
    max_similarity_positive: f64,

    /// Minimum similarity between anchor and negative
    #[arg(long, default_value_t = 0.0)]
    min_similarity_negative: f64,

    /// Maximum similarity between anchor and negative
    #[arg(long, default_value_t = 0.8)]
    max_similarity_negative: f64,
}

#[derive(Parser, Clone, Debug)]
struct ArtificialCommand {
    /// Common parameters
    #[command(flatten)]
    common: Common,
}

#[derive(Parser, Clone, Debug)]
struct DatasetCommand {
    /// Path to the input file
    #[arg(long)]
    input: PathBuf,

    /// Format of the input file
    #[arg(long, value_enum)]
    file_format: FileFormat,

    /// Optional path to file with excluded indexes
    #[arg(long)]
    exclude: Option<PathBuf>,

    /// Common parameters
    #[command(flatten)]
    common: Common,
}

#[derive(ValueEnum, Eq, PartialEq, Clone, Debug)]
enum FileFormat {
    Fasta,
    Fastq,
}

// region Artificial

fn mutate_raw_sequence(generator: &mut StdRng, sequence: String, indexes: &[usize]) -> String {
    let mut chars: Vec<char> = sequence.chars().collect();

    for &index in indexes {
        if index < chars.len() {
            let mut random = chars[index];
            while random == chars[index] {
                random = ALPHABET[generator.gen_range(0..ALPHABET.len())];
            }

            chars[index] = random;
        }
    }

    chars.into_iter().collect()
}

fn generate_raw_sequence(generator: &mut StdRng, state: &mut Vec<usize>, length: usize) -> String {
    state.shuffle(generator);

    let sequence: String = (0..length)
        .map(|_| ALPHABET[generator.gen_range(0..ALPHABET.len())])
        .collect();

    sequence
}

fn create_artificial_neural_dataset(
    generator: &mut StdRng,
    path: String,
    length: usize,
    amount: usize,
    similarity_positive: (usize, usize),
    similarity_negative: (usize, usize),
) -> IoResult<()> {
    let mut state: Vec<usize> = (0..length).collect();
    let mut writer = CsvWriter::from_path(path)?;

    writer.write_record(&["anchor", "positive", "negative"])?;

    for _ in 0..amount {
        let anchor = generate_raw_sequence(generator, &mut state, length);

        state.shuffle(generator);
        let threshold: usize = generator.gen_range(similarity_positive.0..similarity_positive.1);
        let positive = mutate_raw_sequence(generator, anchor.clone(), &state[threshold..]);

        state.shuffle(generator);
        let threshold: usize = generator.gen_range(similarity_negative.0..similarity_negative.1);
        let negative = mutate_raw_sequence(generator, anchor.clone(), &state[threshold..]);

        writer.write_record(&[anchor, positive, negative])?;
    }

    Ok(())
}

fn create_artificial_experiments_dataset(
    generator: &mut StdRng,
    path: String,
    length: usize,
    amount: usize,
) -> IoResult<()> {
    let mut state: Vec<usize> = (0..length).collect();

    let file = File::create(path)?;
    let mut writer = FastaWriter::new(file, None);

    for id in 0..amount {
        let raw_sequence = generate_raw_sequence(generator, &mut state, length);
        let sequence = Sequence::new(&raw_sequence);
        let record = FastaRecord::new(&id.to_string(), None, sequence);
        writer.write(&record)?;
    }

    Ok(())
}

// endregion

// region Dataset

fn generate_unique_random(
    generator: &mut StdRng,
    exclude: &mut HashSet<usize>,
    amount: usize,
    min_value: usize,
    max_value: usize,
) -> HashSet<usize> {
    let mut result = HashSet::new();

    while result.len() != amount {
        let x = generator.gen_range(min_value..max_value);

        if !exclude.contains(&x) {
            result.insert(x);
            exclude.insert(x);
        }
    }

    result
}

fn read_records(path: &Path, format: &FileFormat, ids: &HashSet<usize>) -> IoResult<Vec<Sequence>> {
    let file = File::open(path)?;
    let buffer = BufReader::with_capacity(4000000, file);

    let mut result: Vec<Sequence> = Vec::new();

    match format {
        FileFormat::Fasta => {
            for (idx, record) in FastaReader::new(buffer).iter().enumerate() {
                if ids.contains(&idx) {
                    result.push(record?.sequence().clone());
                }
            }
        }
        FileFormat::Fastq => {
            for (idx, record) in FastqReader::new(buffer).iter().enumerate() {
                if ids.contains(&idx) {
                    result.push(record?.sequence().clone());
                }
            }
        }
    };

    Ok(result)
}

fn create_real_neural_dataset(
    generator: &mut StdRng,
    input: &Path,
    output: &Path,
    file_format: &FileFormat,
    ids: &HashSet<usize>,
    length: usize,
    similarity_positive: (usize, usize),
    similarity_negative: (usize, usize),
) -> IoResult<()> {
    let mut state: Vec<usize> = (0..length).collect();

    let mut sequences = read_records(input, file_format, &ids)?;
    sequences.shuffle(generator);

    let mut writer = CsvWriter::from_path(output)?;
    writer.write_record(&["anchor", "positive", "negative"])?;

    for idx in (0..ids.len()) {
        if idx % 1000 == 0 {
            println!("{} / {}", idx, ids.len())
        }

        let anchor = &sequences[idx];

        state.shuffle(generator);
        let threshold: usize = generator.gen_range(similarity_positive.0..similarity_positive.1);
        let positive =
            mutate_raw_sequence(generator, anchor.content().to_string(), &state[threshold..]);

        state.shuffle(generator);
        let threshold: usize = generator.gen_range(similarity_negative.0..similarity_negative.1);
        let negative =
            mutate_raw_sequence(generator, anchor.content().to_string(), &state[threshold..]);

        writer.write_record(&[anchor.content(), &positive, &negative])?;
    }

    Ok(())
}

fn create_real_experiments_dataset(
    input: &PathBuf,
    output: &PathBuf,
    file_format: &FileFormat,
    ids: &HashSet<usize>,
) -> IoResult<()> {
    let sequences = read_records(input, file_format, &ids)?;

    let file = File::create(output)?;
    let buffer = BufWriter::with_capacity(4000000, file);
    let mut writer = FastaWriter::new(buffer, None);

    for (idx, sequence) in sequences.iter().enumerate() {
        writer.write(&FastaRecord::new(&idx.to_string(), None, sequence.clone()))?
    }

    Ok(())
}

fn save_ids(path: &Path, p1: &HashSet<usize>) -> IoResult<()> {
    let file = File::create(path)?;
    let mut buffer = BufWriter::with_capacity(4000000, file);
    let json = serde_json::to_string(&p1)?;
    buffer.write_all(json.as_bytes())?;

    Ok(())
}

// endregion

fn main() {
    let cli = Cli::parse();

    match cli.cmd {
        Commands::Artificial(cmd) => artificial_command(&cmd),
        Commands::Dataset(cmd) => dataset_command(&cmd),
    };
}

fn artificial_command(args: &ArtificialCommand) {
    let mut generator = StdRng::seed_from_u64(args.common.seed);

    let similarity_positive = (
        (args.common.min_similarity_positive * args.common.length as f64) as usize,
        (args.common.max_similarity_positive * args.common.length as f64) as usize,
    );
    let similarity_negative = (
        (args.common.min_similarity_negative * args.common.length as f64) as usize,
        (args.common.max_similarity_negative * args.common.length as f64) as usize,
    );

    if args.common.training != 0 {
        create_artificial_neural_dataset(
            &mut generator,
            format!("{}/training.csv", args.common.output),
            args.common.length,
            args.common.training,
            similarity_positive,
            similarity_negative,
        )
        .expect("Cannot generate training set");
    }

    if args.common.validation != 0 {
        create_artificial_neural_dataset(
            &mut generator,
            format!("{}/validation.csv", args.common.output),
            args.common.length,
            args.common.validation,
            similarity_positive,
            similarity_negative,
        )
        .expect("Cannot generate validation set");
    }

    if args.common.experiments != 0 {
        create_artificial_experiments_dataset(
            &mut generator,
            format!(
                "{}/{}.fasta",
                args.common.output,
                args.common
                    .experiments_file_name
                    .clone()
                    .unwrap_or("experiments".into())
            ),
            args.common.length,
            args.common.experiments,
        )
        .expect("Cannot generate experiments set");
    }
}

fn dataset_command(args: &DatasetCommand) {
    let mut generator = StdRng::seed_from_u64(args.common.seed);

    let file = File::open(&args.input).expect("Cannot open input dataset");
    let count = FastqReader::new(BufReader::with_capacity(4000000, file))
        .iter()
        .count();

    let mut exclude: HashSet<usize> = match &args.exclude {
        None => HashSet::new(),
        Some(path) => {
            let list: Vec<usize> = serde_json::from_reader(BufReader::new(
                File::open(path).expect("Cannot open exclude file"),
            ))
            .expect("Cannot parse exclude file");
            list.iter().cloned().collect()
        }
    };

    let similarity_positive = (
        (args.common.min_similarity_positive * args.common.length as f64) as usize,
        (args.common.max_similarity_positive * args.common.length as f64) as usize,
    );
    let similarity_negative = (
        (args.common.min_similarity_negative * args.common.length as f64) as usize,
        (args.common.max_similarity_negative * args.common.length as f64) as usize,
    );

    if args.common.training != 0 {
        let ids =
            generate_unique_random(&mut generator, &mut exclude, args.common.training, 0, count);

        create_real_neural_dataset(
            &mut generator,
            &args.input,
            &PathBuf::from(format!("{}/training.csv", args.common.output)),
            &args.file_format,
            &ids,
            args.common.length,
            similarity_positive,
            similarity_negative,
        )
        .expect("Cannot create training dataset");

        save_ids(
            &PathBuf::from(format!("{}/training.exclude", args.common.output)),
            &ids,
        )
        .expect("Cannot save training dataset included ids");
    }

    if args.common.validation != 0 {
        let ids = generate_unique_random(
            &mut generator,
            &mut exclude,
            args.common.validation,
            0,
            count,
        );

        create_real_neural_dataset(
            &mut generator,
            &args.input,
            &PathBuf::from(format!("{}/validation.csv", args.common.output)),
            &args.file_format,
            &ids,
            args.common.length,
            similarity_positive,
            similarity_negative,
        )
        .expect("Cannot create validation dataset");

        save_ids(
            &PathBuf::from(format!("{}/validation.exclude", args.common.output)),
            &ids,
        )
        .expect("Cannot save validation dataset included ids");
    }

    if args.common.experiments != 0 {
        let ids = generate_unique_random(
            &mut generator,
            &mut exclude,
            args.common.experiments,
            0,
            count,
        );

        create_real_experiments_dataset(
            &args.input,
            &PathBuf::from(format!(
                "{}/{}.fasta",
                args.common.output,
                args.common
                    .experiments_file_name
                    .clone()
                    .unwrap_or("experiments".into())
            )),
            &args.file_format,
            &ids,
        )
        .expect("Cannot create experiments dataset");

        save_ids(
            &PathBuf::from(format!("{}/experiments.exclude", args.common.output)),
            &ids,
        )
        .expect("Cannot save experiments dataset included ids");
    }

    save_ids(
        &PathBuf::from(format!("{}/.exclude", args.common.output)),
        &exclude,
    )
    .expect("Cannot save exclude ids");
}

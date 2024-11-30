use clap::Parser;
use csv::Writer;
use exquisitor_core::clustering::ALPHABET;
use rand::prelude::{SliceRandom, StdRng};
use rand::{Rng, SeedableRng};
use std::io::Result as IoResult;

#[derive(Parser, Debug)]
#[command(
    name = "generate",
    version = "0.1.0",
    author = "Patryk Filip Gryz",
    about = "Generate training examples"
)]
struct Cli {
    /// Seed for random generator
    #[arg(long, default_value_t = 152754665)]
    seed: u64,

    /// Length of the sequences
    #[arg(long, default_value_t = 1000)]
    length: usize,

    /// Minimum similarity between anchor and positvie
    #[arg(long, default_value_t = 0.6)]
    min_similarity_positive: f64,

    /// Maximum similarity between anchor and positive
    #[arg(long, default_value_t = 0.9)]
    max_similarity_positive: f64,

    /// Minimum similarity between anchor and negative
    #[arg(long, default_value_t = 0.1)]
    min_similarity_negative: f64,

    /// Maximum similarity between anchor and negative
    #[arg(long, default_value_t = 0.4)]
    max_similarity_negative: f64,

    /// Size of training set
    #[arg(long, default_value_t = 10000)]
    training: usize,

    /// Size of validation set
    #[arg(long, default_value_t = 1000)]
    validation: usize,

    /// Path to output directory
    #[arg(long, default_value = "data")]
    output: String,
}

fn mutate_sample(generator: &mut StdRng, sample: String, indexes: &[usize]) -> String {
    let mut chars: Vec<char> = sample.chars().collect();

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

fn create_samples(
    generator: &mut StdRng,
    path: String,
    length: usize,
    amount: usize,
    similarity_positive: (usize, usize),
    similarity_negative: (usize, usize),
) -> IoResult<()> {
    let mut state: Vec<usize> = (0..length).collect();
    let mut writer = Writer::from_path(path)?;

    writer.write_record(&["anchor", "positive", "negative"])?;

    for _ in 0..amount {
        let anchor: String = (0..length)
            .map(|_| ALPHABET[generator.gen_range(0..ALPHABET.len())])
            .collect();

        state.shuffle(generator);
        let threshold: usize = generator.gen_range(similarity_positive.0..similarity_positive.1);
        let positive = mutate_sample(generator, anchor.clone(), &state[threshold..]);

        state.shuffle(generator);
        let threshold: usize = generator.gen_range(similarity_negative.0..similarity_negative.1);
        let negative = mutate_sample(generator, anchor.clone(), &state[threshold..]);

        writer.write_record(&[anchor, positive, negative])?;
    }

    Ok(())
}

fn main() {
    let cli = Cli::parse();

    let mut generator = StdRng::seed_from_u64(cli.seed);

    let similarity_positive = (
        (cli.min_similarity_positive * cli.length as f64) as usize,
        (cli.max_similarity_positive * cli.length as f64) as usize,
    );
    let similarity_negative = (
        (cli.min_similarity_negative * cli.length as f64) as usize,
        (cli.max_similarity_negative * cli.length as f64) as usize,
    );

    create_samples(
        &mut generator,
        format!("{}/training.csv", cli.output),
        cli.length,
        cli.training,
        similarity_positive,
        similarity_negative,
    )
    .expect("Cannot generate training set");
    create_samples(
        &mut generator,
        format!("{}/validation.csv", cli.output),
        cli.length,
        cli.validation,
        similarity_positive,
        similarity_negative,
    )
    .expect("Cannot generate validation set");
}

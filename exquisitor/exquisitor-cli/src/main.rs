use clap::{Parser, Subcommand, ValueEnum};
use exquisitor_core::clustering::cluster::NaiveClustering;
use exquisitor_core::clustering::distance::{distance_matrix, KMer, NeedlemanWunsch};
use exquisitor_core::clustering::traits::Clustering;
use exquisitor_core::clustering::ALPHABET;
use exquisitor_core::io::fasta::reader::FastaReader;
use exquisitor_core::io::fastq::reader::FastqReader;
use exquisitor_core::io::sequence::Sequence;
use exquisitor_core::io::traits::{Reader, Record};
use exquisitor_core::searching::blast::Blast;
use exquisitor_core::searching::organism::{filter_matches, OrganismFound};
use exquisitor_core::searching::traits::DatabaseSearch;
use std::collections::HashMap;
use std::fs::File;
use std::process;
use tracing::{debug, info};

#[derive(Parser, Debug)]
#[command(name = "run", about = "Runs the given pipeline")]
struct Cli {
    /// Path to the input sequences file
    #[arg(short, long)]
    input: String,

    /// Format of input file
    #[arg(long, value_enum, default_value_t = FileFormat::Auto)]
    file_format: FileFormat,

    /// Pipeline
    #[command(subcommand)]
    pipeline: Pipeline,
}

#[derive(ValueEnum, Clone, Debug)]
enum FileFormat {
    Fasta,
    Fastq,
    Auto,
}

#[derive(Subcommand, Debug)]
enum Pipeline {
    /// Basic pipeline that uses Needleman-Wunsch alignment score as distance metric
    Basic,

    /// Standard pipeline that uses K-mer embedding to compare sequences.
    Standard {
        /// k parameter for K-Mer distance
        #[arg(short, default_value_t = 2)]
        k: usize,
    },

    /// Advanced pipeline that uses neural network for sequence embeddings in clustering
    NeuralNetwork,
}

fn main() {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();

    let format = match cli.file_format {
        FileFormat::Auto => {
            if cli.input.ends_with(".fastq") {
                info!("Detected FASTQ file format by extension");
                FileFormat::Fastq
            } else if cli.input.ends_with(".fasta") {
                info!("Detected FASTA file format by extension");
                FileFormat::Fasta
            } else {
                eprintln!(
                    "Cannot recognize file format. Specify file format using --file-format option"
                );
                process::exit(1);
            }
        }
        FileFormat::Fasta => FileFormat::Fasta,
        FileFormat::Fastq => FileFormat::Fastq,
    };

    let file = File::open(&cli.input);

    if file.is_err() {
        eprintln!("Cannot open file: {}", cli.input);
        process::exit(1);
    }
    let file = file.unwrap();

    let sequences: Vec<Sequence> = match format {
        FileFormat::Fasta => FastaReader::new(file)
            .iter()
            .map(|record| record.unwrap())
            .map(|record| record.sequence().clone())
            .collect(),
        FileFormat::Fastq => FastqReader::new(file)
            .iter()
            .map(|record| record.unwrap())
            .map(|record| record.sequence().clone())
            .collect(),
        FileFormat::Auto => {
            process::exit(1);
        }
    };

    let result = match cli.pipeline {
        Pipeline::Basic => {
            run_basic_pipeline(sequences)
        }
        Pipeline::Standard { k } => {
            run_standard_pipeline(sequences, k)
        }
        Pipeline::NeuralNetwork => {
            println!("Not implemented yet!");
            process::exit(0);
        }
    };

    info!("Found {}", result.len());
    for found in result {
        debug!("- {}", found.name());
    }
}

fn run_basic_pipeline(sequences: Vec<Sequence>) -> Vec<OrganismFound> {
    let mut identity_matrix = HashMap::new();
    for &x in ALPHABET {
        for &y in ALPHABET {
            let value = if x == y { 1f64 } else { -1f64 };
            identity_matrix.insert((x, y), value);
        }
    }

    let distance_matrix = distance_matrix(&sequences, &NeedlemanWunsch::new(-8f64, identity_matrix));
    if distance_matrix.is_err() {
        eprintln!("Cannot calculate distance matrix {}", distance_matrix.unwrap_err().message());
        process::exit(1);
    }
    let distance_matrix = distance_matrix.unwrap();

    let clustering_method = NaiveClustering::new(1f64);
    let clusters = clustering_method.cluster(distance_matrix);
    if clusters.is_err() {
        eprintln!("Cannot calculate cluster matrix {}", clusters.unwrap_err().message());
        process::exit(1);
    }
    let clusters = clusters.unwrap();

    info!("Clusters: {}", clusters.len());

    let representatives = clusters.
        iter()
        .filter_map(|c| sequences.get(c.representative()))
        .cloned()
        .collect();

    let database = Blast::new("/blast/blastn", "/blast/db");
    let matches = database.search(representatives);
    if matches.is_err() {
        eprintln!("Cannot search sequences in database");
        process::exit(1);
    }
    let matches = matches.unwrap();

    let found = filter_matches(&matches, &clusters, sequences.len());
    found
}

fn run_standard_pipeline(sequences: Vec<Sequence>, k: usize) -> Vec<OrganismFound> {
    let distance_metric = KMer::new(k);
    let distance_matrix = distance_matrix(&sequences, &distance_metric);
    if distance_matrix.is_err() {
        eprintln!("Cannot calculate distance matrix {}", distance_matrix.unwrap_err().message());
        process::exit(1);
    }
    let distance_matrix = distance_matrix.unwrap();

    let clustering_method = NaiveClustering::new(1f64);
    let clusters = clustering_method.cluster(distance_matrix);
    if clusters.is_err() {
        eprintln!("Cannot calculate cluster matrix {}", clusters.unwrap_err().message());
        process::exit(1);
    }
    let clusters = clusters.unwrap();

    info!("Clusters: {}", clusters.len());

    let representatives = clusters.
        iter()
        .filter_map(|c| sequences.get(c.representative()))
        .cloned()
        .collect();

    let database = Blast::new("/blast/blastn", "/blast/db");
    let matches = database.search(representatives);
    if matches.is_err() {
        eprintln!("Cannot search sequences in database");
        process::exit(1);
    }
    let matches = matches.unwrap();

    let found = filter_matches(&matches, &clusters, sequences.len());
    found
}
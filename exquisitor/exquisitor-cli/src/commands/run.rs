use burn::backend::wgpu::WgpuDevice;
use burn::backend::Wgpu;
use clap::{Parser, ValueEnum};
use exquisitor_core::clustering::cluster::{
    save_clustering_data, KMedoidClustering, NaiveClustering,
};
use exquisitor_core::clustering::distance::{
    distance_matrix, CosineDistance, DistanceMatrix, KMer, NeedlemanWunsch, SimilarityMatrix,
};
use exquisitor_core::clustering::neural::NeuralEmbedder;
use exquisitor_core::clustering::traits::Clustering;
use exquisitor_core::clustering::ALPHABET;
use exquisitor_core::io::fasta::reader::FastaReader;
use exquisitor_core::io::fastq::reader::FastqReader;
use exquisitor_core::io::sequence::Sequence;
use exquisitor_core::io::traits::{Reader, Record};
use exquisitor_core::searching::blast::Blast;
use exquisitor_core::searching::organism::{filter_matches, save_found_organisms};
use exquisitor_core::searching::traits::DatabaseSearch;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;
use std::fs::File;
use std::io::Error as IoError;
use std::io::ErrorKind;
use std::io::Result as IoResult;
use std::path::PathBuf;
use tracing::{debug, info};

#[derive(Parser, Debug, Clone)]
pub(crate) struct RunCommand {
    /// Path to the input sequence file
    #[arg(short, long)]
    input: PathBuf,

    /// Path to the output file
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// File format of the input file
    #[arg(long, value_enum, default_value_t = FileFormat::Auto)]
    file_format: FileFormat,

    /// Pipeline
    #[arg(long, value_enum)]
    pipeline: Pipeline,

    /// Clustering configuration
    #[command(flatten)]
    clustering_configuration: ClusteringConfiguration,

    /// Print only clustering results
    #[arg(long, action)]
    only_cluster: bool,

    /// Database configuration
    #[command(flatten)]
    database_configuration: DatabaseConfiguration,

    /// Logging
    #[arg(long, default_value = "info")]
    log_level: String,
}

#[derive(Parser, Debug, Clone)]
struct ClusteringConfiguration {
    /// Method used for clustering
    #[arg(long)]
    clustering: ClusteringMethod,

    /// Gap penalty modifier used in Needleman-Wunsch algorithm
    #[arg(long, required_if_eq("pipeline", "basic"), allow_hyphen_values = true)]
    gap_penalty: Option<f64>,

    /// Similarity matrix used in Needleman-Wunsch algorithm
    #[arg(long)]
    similarity_matrix_file: Option<PathBuf>,

    /// K parameter used in KMer algorithm
    #[arg(long, required_if_eq_any([("pipeline", "kmer"), ("clustering", "kmedoid")]))]
    k: Option<usize>,

    /// Path to neural model
    #[arg(long, required_if_eq("pipeline", "neural"))]
    model: Option<String>,

    /// Max distance between clusters
    #[arg(long, required_if_eq("clustering", "naive"))]
    max_distance: Option<f64>,
}

#[derive(Parser, Debug, Clone)]
struct DatabaseConfiguration {
    /// Path to BLAST database executable
    #[arg(long)]
    blast: PathBuf,

    /// Path to BLAST database files
    #[arg(long)]
    blast_db: PathBuf,
}

#[derive(ValueEnum, Eq, PartialEq, Clone, Debug)]
enum FileFormat {
    Fasta,
    Fastq,
    Auto,
}

impl fmt::Display for FileFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                FileFormat::Fasta => "FASTA",
                FileFormat::Fastq => "FASTQ",
                FileFormat::Auto => "Unknown",
            }
        )
    }
}

#[derive(ValueEnum, Clone, Debug)]
enum Pipeline {
    Basic,
    KMer,
    Neural,
}

#[derive(ValueEnum, Clone, Debug)]
enum ClusteringMethod {
    Naive,
    KMedoid,
}

/// Handle run command
pub(crate) fn run(args: RunCommand) -> IoResult<()> {
    // Initialize tracing
    let severity = args
        .log_level
        .parse::<tracing::Level>()
        .unwrap_or(tracing::Level::INFO);
    tracing_subscriber::fmt().with_max_level(severity).init();

    // Detect file format
    let format = match args.file_format {
        FileFormat::Auto => detect_file_format(&args.input)?,
        other => other,
    };

    debug!("File format: {}", format.to_string());

    // Load sequences
    let sequences = load_sequences(&args.input, format)?;

    debug!("Loaded {} sequences", sequences.len());

    // Distance matrix
    let distance_matrix = match args.pipeline {
        Pipeline::Basic => {
            let gap_penalty = args
                .clustering_configuration
                .gap_penalty
                .ok_or(IoError::new(
                    ErrorKind::Other,
                    "Missing gap penalty modifier",
                ))?;

            let similarity_matrix = create_similarity_matrix();

            distance_matrix(
                &sequences,
                &NeedlemanWunsch::new(gap_penalty, similarity_matrix),
            )?
        }
        Pipeline::KMer => {
            let distance_metric = KMer::new(args.clustering_configuration.k.ok_or(
                IoError::new(ErrorKind::Other, "Missing k parameter for KMer algorithm"),
            )?);

            distance_matrix(&sequences, &distance_metric)?
        }
        Pipeline::Neural => {
            let device: WgpuDevice = Default::default();
            let embedder = NeuralEmbedder::<Wgpu<f32, i32>>::new(
                &args.clustering_configuration.model.ok_or(IoError::new(
                    ErrorKind::Other,
                    "Missing path to neural model",
                ))?,
                device.clone(),
            )?;
            debug!("Neural model loaded!");

            let embeddings = embedder.embed(device.clone(), &sequences);
            debug!("Embeddings ready!");

            let embeddings = embeddings
                .iter_dim(0)
                .map(|t| t.to_data().to_vec::<f32>().unwrap())
                .collect::<Vec<_>>();

            for embed in &embeddings {
                debug!("{:?}", embed);
            }

            distance_matrix(&embeddings, &CosineDistance)?
        }
    };

    for line in &distance_matrix {
        println!("{:?}", line);
    }

    debug!("Calculated distance matrix");

    let clustering_method: Box<dyn Clustering<DistanceMatrix>> =
        match args.clustering_configuration.clustering {
            ClusteringMethod::Naive => Box::new(NaiveClustering::new(
                args.clustering_configuration
                    .max_distance
                    .ok_or(IoError::new(
                        ErrorKind::Other,
                        "Missing max distance parameter",
                    ))?,
            )),
            ClusteringMethod::KMedoid => Box::new(KMedoidClustering::new(
                args.clustering_configuration.k.ok_or(IoError::new(
                    ErrorKind::Other,
                    "Missing k parameter for KMedoids clustering",
                ))?,
            )),
        };

    let clusters = clustering_method.cluster(distance_matrix)?;

    debug!("Clustered into {}", clusters.len());

    if args.only_cluster {
        if let Some(path) = args.output {
            let mut file = File::create(path.clone())?;
            save_clustering_data(&mut file, &clusters)?;

            debug!("Saved clusters to {}", path.to_string_lossy());
        }

        return Ok(());
    }

    let representatives = clusters
        .iter()
        .filter_map(|c| sequences.get(c.representative()))
        .cloned()
        .collect();

    let database = Blast::new(
        args.database_configuration.blast.to_str().unwrap(),
        args.database_configuration.blast_db.to_str().unwrap(),
    );
    let matches = database.search(representatives)?;
    let found = filter_matches(&matches, &clusters, sequences.len());

    if let Some(path) = args.output {
        let mut file = File::create(path.clone())?;
        save_found_organisms(&mut file, &found)?;

        debug!("Saved result to {}", path.to_string_lossy());
    } else {
        info!("Found {}", found.len());
        for found in found {
            info!("- {}", found.name());
        }
    }

    Ok(())
}

/// Detect file format
fn detect_file_format(path: &PathBuf) -> IoResult<FileFormat> {
    match path.extension() {
        Some(extension) if extension.eq_ignore_ascii_case("fasta") => Ok(FileFormat::Fasta),
        Some(extension) if extension.eq_ignore_ascii_case("fastq") => Ok(FileFormat::Fastq),
        _ => Err(IoError::new(
            ErrorKind::NotFound,
            "Cannot recognize file format. Specify file format using --file-format option",
        )),
    }
}

/// Load records
fn load_sequences(path: &PathBuf, format: FileFormat) -> IoResult<Vec<Sequence>> {
    let file = File::open(&path)?;

    let sequences: IoResult<Vec<Sequence>> = match format {
        FileFormat::Fasta => FastaReader::new(file)
            .iter()
            .map(|record| record.map(|value| value.sequence().clone()))
            .collect(),
        _ => FastqReader::new(file)
            .iter()
            .map(|record| record.map(|value| value.sequence().clone()))
            .collect(),
    };

    sequences
}

/// Similarity matrix
fn create_similarity_matrix() -> SimilarityMatrix {
    let mut similarity_matrix = HashMap::new();

    for &x in ALPHABET {
        for &y in ALPHABET {
            let value = if x == y { 0f64 } else { 1f64 };
            similarity_matrix.insert((x, y), value);
        }
    }

    similarity_matrix
}

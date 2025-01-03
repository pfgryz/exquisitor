use burn::backend::wgpu::WgpuDevice;
use burn::backend::Wgpu;
use clap::{Parser, ValueEnum};
use exquisitor_core::clustering::cluster::{
    save_clustering_data, KMedoidClustering, NaiveClustering,
};
use exquisitor_core::clustering::distance::{
    distance_matrix, CosineDistance, DistanceMatrix, KMer, NeedlemanWunsch,
};
use exquisitor_core::clustering::neural::NeuralEmbedder;
use exquisitor_core::clustering::traits::Clustering;
use exquisitor_core::io::fasta::reader::FastaReader;
use exquisitor_core::io::fastq::reader::FastqReader;
use exquisitor_core::io::sequence::Sequence;
use exquisitor_core::io::traits::{Reader, Record};
use exquisitor_core::searching::blast::Blast;
use exquisitor_core::searching::organism::{filter_matches, save_found_organisms, save_matches};
use exquisitor_core::searching::traits::DatabaseSearch;
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

    /// Save clustering information
    #[arg(long, action)]
    save_clusters: bool,

    /// Database configuration
    #[command(flatten)]
    database_configuration: DatabaseConfiguration,
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

    /// Number of clusters
    #[arg(long, required_if_eq_any([("clustering", "kmedoid")]))]
    k: Option<usize>,

    /// K parameter used in KMer algorithm
    #[arg(long, required_if_eq_any([("pipeline", "kmer")]))]
    kmer: Option<usize>,

    /// Path to neural model
    #[arg(long, required_if_eq("pipeline", "neural"))]
    model: Option<String>,

    /// Max distance between clusters
    #[arg(long, required_if_eq("clustering", "naive"))]
    max_distance: Option<f64>,
}

#[derive(Parser, Debug, Clone)]
pub(crate) struct DatabaseConfiguration {
    /// Path to BLAST database executable
    #[arg(long)]
    pub(crate) blast: PathBuf,

    /// Path to BLAST database files
    #[arg(long)]
    pub(crate) blast_db: PathBuf,
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

/// Run full pipeline of taxonomic classification with clustering and preprocessing
pub(crate) fn run(args: RunCommand) -> IoResult<()> {
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

            let similarity_matrix = NeedlemanWunsch::create_default_similarity_matrix();

            distance_matrix(
                &sequences,
                &NeedlemanWunsch::new(gap_penalty, similarity_matrix),
            )?
        }
        Pipeline::KMer => {
            let distance_metric = KMer::new(args.clustering_configuration.kmer.ok_or(
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

            distance_matrix(&embeddings, &CosineDistance)?
        }
    };

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

    if args.only_cluster || args.save_clusters {
        if let Some(ref path) = args.output {
            let mut clusters_path = path.clone();
            clusters_path.set_extension("clusters".to_string());
            let mut file = File::create(&clusters_path)?;
            save_clustering_data(&mut file, &clusters)?;

            debug!("Saved clusters to {}", clusters_path.to_string_lossy());
        }
    }

    if args.only_cluster {
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

    if let Some(ref path) = args.output {
        let mut matches_path = path.clone();
        matches_path.set_extension("matches".to_string());
        let mut file = File::create(&matches_path)?;
        save_matches(&mut file, &matches)?;
    }
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

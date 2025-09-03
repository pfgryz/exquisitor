use std::env;
use std::fs::File;
use std::path::PathBuf;
use burn::backend::Wgpu;
use std::io::Result as IoResult;
use burn::backend::wgpu::WgpuDevice;
use csv::{Reader, Writer};
use serde::{Deserialize, Serialize};
use tracing::{info, Level};
use exquisitor_core::clustering::cluster::{Cluster, KMedoidClustering};
use exquisitor_core::clustering::dissimilarity::{dissimilarity_matrix, CosineDissimilarity};
use exquisitor_core::clustering::neural::NeuralEmbedder;
use exquisitor_core::clustering::traits::Clustering;
use exquisitor_core::io::sequence::Sequence;

fn main() {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("Usage: {} <model_path> <sample_path> <clusters>", args[0]);
        std::process::exit(1);
    }

    let sample_path: PathBuf = args[2].clone().into();
    let sequences_path: PathBuf = sample_path.join("for_grouping.csv");
    let output_path: PathBuf = sample_path.join("groups.csv");
    let k: usize = args[3].parse().expect("Number of clusters must be a positive number");

    // Load sequences
    info!("Loading sequences...!");
    let sequences = load_sequences(&sequences_path).unwrap();

    // Load model
    info!("Loading model...!");
    let device: WgpuDevice = WgpuDevice::DiscreteGpu(0);
    let embedder = NeuralEmbedder::<Wgpu<f32, i32>>::new(
        &args[1],
        device.clone()
    ).unwrap();

    // Prepare embeddings
    info!("Preparing embeddings...!");
    let embeddings = sequences.chunks(256)
        .flat_map(|batch| embedder.embed(device.clone(), &batch.to_vec()).iter_dim(0).map(|t| t.to_data().to_vec::<f32>().unwrap()).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // Create dissimilarity matrix
    info!("Creating dissimilarity matrix...!");
    let matrix = dissimilarity_matrix(&embeddings, &CosineDissimilarity).unwrap();

    // Cluster sequences
    info!("Clustering sequences...!");
    let clustering_method = KMedoidClustering::new(
        k
    );
    let clusters = clustering_method.cluster(matrix).unwrap();

    // Save results
    info!("Saving..!");
    save_clusters(&clusters, &sequences, &output_path);
}

#[derive(Serialize)]
struct ClusterRecord {
    cluster_id: usize,
    sequence: String,
}

fn save_clusters(clusters: &Vec<Cluster>, sequences: &Vec<Sequence>, output_path: &PathBuf) {
    let file = File::create(output_path).unwrap();
    let mut writer = Writer::from_writer(file);

    for (cluster_id, cluster) in clusters.iter().enumerate() {
        for &member in cluster.sequence_ids() {
            let record = ClusterRecord {
                cluster_id,
                sequence: sequences.get(member).unwrap().content().into()
            };
            writer.serialize(record).unwrap();
        }
    }

    writer.flush().unwrap();
}

#[derive(Debug, Deserialize)]
struct Record {
    id: String,
    sequence: String,
    tax_id: String
}

fn load_sequences(path: &PathBuf) -> IoResult<Vec<Sequence>> {
    let file = File::open(path)?;

    let mut rdr = Reader::from_reader(file);

    Ok(rdr.deserialize::<Record>()
        .into_iter()
        .map(|r| r.unwrap())
        .map(|r| Sequence::new(&r.sequence))
        .collect::<Vec<_>>())
}
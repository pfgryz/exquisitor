use burn::serde::Serialize;
use clap::Parser;
use exquisitor_core::clustering::cluster::{
    clusters_fmi_score, clusters_nmi_score, load_clustering_data, Cluster,
};
use std::fs::File;
use std::io::{Result as IoResult, Write};
use std::path::{Path, PathBuf};
use tracing::debug;

#[derive(Parser, Debug, Clone)]
pub(crate) struct CompareClustersCommand {
    /// Path to the reference
    #[arg(long)]
    reference: PathBuf,

    /// Path to the second file
    #[arg(long)]
    second: PathBuf,

    /// Path to the output file
    #[arg(long)]
    output: Option<PathBuf>,
}

#[derive(Serialize)]
struct CompareClustersResult {
    reference: PathBuf,
    second: PathBuf,
    fmi: f64,
    nmi: f64,
}

fn load_clusters(path: &Path) -> IoResult<Vec<Cluster>> {
    let mut file = File::open(path)?;
    load_clustering_data(&mut file)
}

/// Compare two sets of clusters with FMI and NMI scores
pub(crate) fn compare_clusters(args: CompareClustersCommand) -> IoResult<()> {
    let first = load_clusters(args.reference.as_path())?;
    let second = load_clusters(args.second.as_path())?;

    debug!("Loaded both clusterings");

    let fmi = clusters_fmi_score(&first, &second);
    let nmi = clusters_nmi_score(&first, &second);

    debug!("Calculated FMI & NMI score");

    if let Some(path) = args.output {
        let mut file = File::create(path)?;
        let json = serde_json::to_string(&CompareClustersResult {
            reference: args.reference,
            second: args.second,
            fmi,
            nmi,
        })?;
        file.write_all(json.as_bytes())?;
    } else {
        println!("FMI: {}; NMI: {};", fmi, nmi);
    }

    Ok(())
}

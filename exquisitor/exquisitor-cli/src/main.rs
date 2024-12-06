mod commands;

use crate::commands::clusters::{compare_clusters, CompareClustersCommand};
use crate::commands::compare::{compare, CompareCommand};
use crate::commands::experiment::{experiment, ExperimentCommand};
use crate::commands::run::{run, RunCommand};
use crate::commands::search::{search, SearchCommand};
use clap::{Parser, Subcommand};
use std::process;

#[derive(Parser, Debug)]
#[command(
    version = "0.1.0",
    author = "Patryk Filip Gryz",
    about = "Simple CLI for running pipelines, experiments and comparing results"
)]
struct Cli {
    #[command(subcommand)]
    cmd: Commands,

    /// Logging
    #[arg(long, default_value = "info")]
    log_level: String,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    /// Run the pipeline
    Run(RunCommand),
    /// Run the experiment
    Experiment(ExperimentCommand),
    /// Compare the results
    Compare(CompareCommand),
    /// Compare the clusters
    CompareClusters(CompareClustersCommand),
    /// Search sequences in database
    Search(SearchCommand),
}

fn main() {
    let cli = Cli::parse();

    // Initialize tracing
    let severity = cli
        .log_level
        .parse::<tracing::Level>()
        .unwrap_or(tracing::Level::INFO);
    tracing_subscriber::fmt().with_max_level(severity).init();

    let result = match cli.cmd {
        Commands::Run(cmd) => run(cmd),
        Commands::Experiment(cmd) => experiment(cmd),
        Commands::Compare(cmd) => compare(cmd),
        Commands::CompareClusters(cmd) => compare_clusters(cmd),
        Commands::Search(cmd) => search(cmd),
    };

    match result {
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
        _ => {}
    }
}

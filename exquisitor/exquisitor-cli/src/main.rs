mod commands;

use crate::commands::compare::{compare, CompareCommand};
use crate::commands::run::{run, RunCommand};
use clap::{Parser, Subcommand, ValueEnum};
use exquisitor_core::clustering::traits::Clustering;
use exquisitor_core::io::traits::{Reader, Record};
use exquisitor_core::searching::traits::DatabaseSearch;
use std::cmp::PartialEq;
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
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    /// Run the pipeline
    Run(RunCommand),
    /// Compare the results
    Compare(CompareCommand),
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.cmd {
        Commands::Run(cmd) => run(cmd),
        Commands::Compare(cmd) => compare(cmd),
    };

    match result {
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
        _ => {}
    }
}

use clap::Parser;
use exquisitor_core::searching::organism::{load_found_organisms, OrganismFound};
use exquisitor_core::searching::quality::calculate_search_quality;
use std::fs::File;
use std::io::Result as IoResult;
use std::path::PathBuf;

#[derive(Parser, Debug, Clone)]
pub(crate) struct CompareCommand {
    /// Path to the reference
    #[arg(long)]
    reference: PathBuf,

    /// Path to the second file
    #[arg(long)]
    second: PathBuf,
}

/// Compare the results of taxonomic classification
pub(crate) fn compare(args: CompareCommand) -> IoResult<()> {
    let reference = load(args.reference)?;
    let second = load(args.second)?;
    let quality = calculate_search_quality(reference, second);
    println!("Pos: {} Neg: {}", quality.0, quality.1);

    Ok(())
}

fn load(path: PathBuf) -> IoResult<Vec<OrganismFound>> {
    let mut file = File::open(path)?;
    load_found_organisms(&mut file)
}

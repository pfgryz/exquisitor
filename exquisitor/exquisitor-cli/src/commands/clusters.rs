use clap::Parser;
use std::io::Result as IoResult;
use std::path::PathBuf;

#[derive(Parser, Debug, Clone)]
pub(crate) struct CompareClustersCommand {
    /// Path to the reference
    #[arg(long)]
    reference: PathBuf,

    /// Path to the second file
    #[arg(long)]
    second: PathBuf,

    /// Method
    #[arg(long)]
    method: String,
}

pub(crate) fn compare_clusters(args: CompareClustersCommand) -> IoResult<()> {
    Ok(())
}

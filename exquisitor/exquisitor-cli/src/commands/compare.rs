use clap::Parser;
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

pub(crate) fn compare(args: CompareCommand) -> IoResult<()> {
    Ok(())
}

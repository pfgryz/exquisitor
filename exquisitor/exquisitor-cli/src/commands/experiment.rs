use clap::Parser;
use std::io::Result as IoResult;

#[derive(Parser, Debug, Clone)]
pub(crate) struct ExperimentCommand {
    /// Resolution
    #[arg(long)]
    resolution: f32,
}

pub(crate) fn experiment(args: ExperimentCommand) -> IoResult<()> {
    Ok(())
}

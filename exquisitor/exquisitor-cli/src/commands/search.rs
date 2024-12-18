use clap::Parser;
use exquisitor_core::searching::blast::Blast;
use std::io::Result as IoResult;
use std::path::PathBuf;

#[derive(Parser, Clone, Debug)]
pub(crate) struct SearchCommand {
    /// Path to the input sequence file
    #[arg(short, long)]
    input: PathBuf,

    /// Path to the output file
    #[arg(short, long)]
    output: PathBuf,

    /// Database configuration
    #[command(flatten)]
    database_configuration: crate::commands::run::DatabaseConfiguration,
}

/// Search the input sequences in BLASTn database
pub(crate) fn search(args: SearchCommand) -> IoResult<()> {
    let database = Blast::new(
        args.database_configuration.blast.to_str().unwrap(),
        args.database_configuration.blast_db.to_str().unwrap(),
    );

    database.search_file(&args.input, &args.output)?;

    Ok(())
}

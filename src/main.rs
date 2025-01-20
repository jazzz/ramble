use anyhow::Result;
use clap::{Parser, Subcommand};
use paris::{info, Logger};

fn main() -> Result<()> {
    info!("Starting Ramble");

    info!("Done");

    Ok(())
}

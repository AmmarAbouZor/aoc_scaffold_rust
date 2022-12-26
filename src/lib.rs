use anyhow::Result;
use clap::Parser;
mod commands;
mod config;

pub fn run() -> Result<()> {
    let cmd = commands::Command::parse();

    cmd.run()?;

    Ok(())
}

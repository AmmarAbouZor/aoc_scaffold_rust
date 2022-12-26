mod config;
mod next_day;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command()]
pub struct Command {
    #[command(subcommand)]
    command: SubCommands,
}

impl Command {
    pub fn run(&self) -> Result<()> {
        match &self.command {
            SubCommands::Config(cmd) => cmd.run(),
            SubCommands::NextDay(cmd) => cmd.run(),
        }
    }
}

#[derive(Subcommand, Debug)]
enum SubCommands {
    Config(config::Command),
    NextDay(next_day::Command),
}

use anyhow::Context;
use anyhow::Result;
use clap::{Args, Subcommand};

use crate::config;

/// Manage the configs
#[derive(Args, Debug)]
#[command(disable_help_subcommand = true)]
pub struct Command {
    #[command(subcommand)]
    command: SubCommands,
}

#[derive(Subcommand, Debug)]
enum SubCommands {
    SetYear(SetYearCommand),
    GetYear(GetYearCommand),
    Open(OpenConfigCommand),
    GetPath(GetPathCommand),
}

impl Command {
    pub fn run(&self) -> Result<()> {
        match &self.command {
            SubCommands::GetPath(cmd) => cmd.run(),
            SubCommands::SetYear(cmd) => cmd.run(),
            SubCommands::GetYear(cmd) => cmd.run(),
            SubCommands::Open(cmd) => cmd.run(),
        }
    }
}

/// return the path of the config file
#[derive(Args, Debug)]
#[command()]
struct GetPathCommand {}

impl GetPathCommand {
    fn run(&self) -> Result<()> {
        eprintln!("{}", config::get_path()?.display());

        Ok(())
    }
}

/// return the current year in the settings
#[derive(Args, Debug)]
#[command()]
struct GetYearCommand {}

impl GetYearCommand {
    fn run(&self) -> Result<()> {
        let conf = config::load()?;

        println!("year is: '{}'", conf.year);

        Ok(())
    }
}

/// set the current year in the settings
#[derive(Args, Debug)]
#[command()]
pub struct SetYearCommand {
    year: String,
}

impl SetYearCommand {
    pub fn new(year: String) -> Self {
        Self { year }
    }

    pub fn run(&self) -> Result<()> {
        let mut conf = config::load()?;

        conf.year = self.year.clone();
        config::save(conf)?;

        Ok(())
    }
}

/// open the config file in the default text editor
#[derive(Args, Debug)]
#[command()]
struct OpenConfigCommand {}

impl OpenConfigCommand {
    fn run(&self) -> Result<()> {
        let path = config::get_path()?;
        edit::edit_file(path).context("couln't open the config file with the default Editor")?;
        Ok(())
    }
}

mod complete;
mod config;
mod next_day;
mod run_tests;

use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand};
use itertools::Itertools;
use std::{env, fs, path::PathBuf};

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Command {
    #[command(subcommand)]
    command: SubCommands,
}

impl Command {
    pub fn run(&self) -> Result<()> {
        match &self.command {
            SubCommands::Config(cmd) => cmd.run(),
            SubCommands::NextDay(cmd) => cmd.run(),
            SubCommands::RunTest(cmd) => cmd.run(),
            SubCommands::Complete(cmd) => cmd.run(),
        }
    }
}

#[derive(Subcommand, Debug)]
enum SubCommands {
    #[clap(visible_alias = "c")]
    Config(config::Command),
    #[clap(visible_alias = "n")]
    NextDay(next_day::Command),
    #[clap(visible_alias = "t")]
    RunTest(run_tests::Command),
    #[clap(visible_alias = "comp")]
    Complete(complete::Command),
}

/// Checks if the current directory is an executable rust project and return it.
///
/// # Errors
///
/// This function will return an error if any of the files 'Cargo.toml' or 'main.rs' doesn't exist or an io Error.
pub(crate) fn get_project_directory() -> Result<PathBuf> {
    let current_directory = env::current_dir()?;
    let cargo_path = current_directory.join("Cargo.toml");
    if !cargo_path.try_exists()? {
        bail!("current directory isn't rust project. 'Cargo.toml' could't be found")
    }

    let main_path = current_directory.join("src").join("main.rs");

    if !main_path.try_exists()? {
        bail!("current directory isn't rust project. 'main.rs' could't be found")
    }

    // TODO delete this and the env VAR from bashrc and fish
    //let current_directory = PathBuf::from(env::var("AOC_TEST_PATH")?);

    Ok(current_directory)
}

pub fn get_year_name(year: &str) -> String {
    format!("year_{year}")
}

pub fn get_day_name(day: u8) -> String {
    format!("day_{:02}", day)
}

pub fn get_last_day(year_dir_path: &PathBuf) -> Result<u8> {
    // filename of the days follow the pattern day_XX.rs
    const DAY_FILE_LENGTH: usize = 9;

    let last_day = fs::read_dir(year_dir_path)?
        .flatten()
        .flat_map(|file_path| file_path.file_name().into_string())
        .filter(|file| {
            file.starts_with("day_") && file.ends_with(".rs") && file.len() == DAY_FILE_LENGTH
        })
        .sorted()
        .last()
        .context("Years folder exists but it's empty")?;

    let last_day_num = last_day[4..6].parse().context(format!(
        "filename {last_day} doesn't follow the pattern day_XX.rs"
    ))?;

    Ok(last_day_num)
}

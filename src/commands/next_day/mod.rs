use anyhow::{bail, Result};
use clap::Args;
use std::{env, io, path::PathBuf};

use crate::{commands::config::SetYearCommand, config};
mod src_scaff;

/// scaffold the the repo incrementing the current day and adding a new year if the current year doesn't existed.
#[derive(Args, Debug)]
#[command()]
pub struct Command {
    /// create an input file for the next day inside the input folder
    #[arg(short, long)]
    input: bool,

    /// open the created input file for the next if it created
    #[arg(short, long)]
    open_input: bool,
}

impl Command {
    pub fn run(&self) -> Result<()> {
        // let current_directory = self.check_current_directory()?;
        // TODO reactivate current directory and delete env VAR from bashrc and fish

        let current_directory = PathBuf::from(env::var("AOC_TEST_PATH")?);
        self.validate_config()?;
        let config = config::load()?;
        src_scaff::scaff_next_day(&current_directory, &config.year)?;
        Ok(())
    }

    /// Checks if the year is set. If not it asks the user to set it and save it to the configs.
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    fn validate_config(&self) -> Result<()> {
        let config = config::load()?;
        if config.year.is_empty() {
            println!(
                "Curent year isn't set yet.\nPlease enter the current year (It can be changed after that from the config command):"
            );

            let mut year = String::new();
            io::stdin().read_line(&mut year)?;
            year = year.trim().into();

            SetYearCommand::new(year).run()?;
        }

        Ok(())
    }

    /// Checks if the current directory is an executable rust project.
    ///
    /// # Errors
    ///
    /// This function will return an error if any of the files 'Cargo.toml' or 'main.rs' doesn't exist or an io Error.
    fn check_current_directory(&self) -> Result<PathBuf> {
        let current_directory = env::current_dir()?;
        let cargo_path = current_directory.join("Cargo.toml");
        if !cargo_path.try_exists()? {
            bail!("current directory isn't rust project. 'Cargo.toml' could't be found")
        }

        let main_path = current_directory.join("src").join("main.rs");

        if !main_path.try_exists()? {
            bail!("current directory isn't rust project. 'main.rs' could't be found")
        }

        Ok(current_directory)
    }
}

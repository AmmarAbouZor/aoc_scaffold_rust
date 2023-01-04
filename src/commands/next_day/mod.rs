use anyhow::{Context, Result};
use clap::Args;
use std::{fs, io};

use crate::{
    commands::{self, config::SetYearCommand},
    config,
};
mod src_scaff;

/// scaffold the the repo incrementing the current day and adding a new year if the current year doesn't exist.
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
        let current_directory = crate::commands::get_project_directory()?;
        self.validate_config()?;
        let config = config::load()?;
        let day_num = src_scaff::scaff_next_day(&current_directory, &config.year)?;
        if self.input {
            let input_dir = current_directory
                .join("input")
                .join(commands::get_year_name(&config.year));
            fs::create_dir_all(&input_dir)?;
            let file_name = input_dir
                .join(commands::get_day_name(day_num))
                .with_extension("txt");
            fs::File::create(&file_name)?;
            println!(
                "Input file '{}' has been created",
                file_name.to_string_lossy()
            );

            if self.open_input {
                edit::edit_file(file_name)
                    .context("couln't open the input file with the default Editor")?;
            }
        }

        Ok(())
    }

    /// Checks if the year is set. If not it asks the user to set it and save it to the configs.
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
}

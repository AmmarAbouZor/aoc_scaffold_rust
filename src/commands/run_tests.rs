use std::process;

use anyhow::Result;
use clap::Args;

use crate::commands;
use crate::config;

/// Run unit tests for the last day only
#[derive(Args, Debug)]
#[command()]
pub struct Command {}

impl Command {
    pub fn run(&self) -> Result<()> {
        let project_directory = commands::get_project_directory()?;
        let year = config::load()?.year;
        let year_name = commands::get_year_name(&year);
        let year_path = project_directory.join("src").join(&year_name);
        dbg!(&year_path);
        let last_day_num = commands::get_last_day(&year_path)?;

        dbg!(last_day_num);
        let test_argument = format!("{}::{}", year_name, commands::get_day_name(last_day_num));

        process::Command::new("cargo")
            .current_dir(project_directory)
            .arg("test")
            .arg(test_argument)
            .status()?;

        Ok(())
    }
}

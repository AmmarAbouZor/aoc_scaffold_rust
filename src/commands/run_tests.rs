use std::process;

use anyhow::Result;
use clap::Args;

use crate::commands;
use crate::config;

/// Run unit tests for the last day only
#[derive(Args, Debug)]
#[command()]
pub struct Command {
    /// run tests in release mode
    #[arg(short, long)]
    release: bool,

    /// add nocapture flage to the test to show prints and debug info (cargo test -- --nocapture)
    #[arg(short, long)]
    nocapture: bool,
}

impl Command {
    pub fn run(&self) -> Result<()> {
        let project_directory = commands::get_project_directory()?;
        let year = config::load()?.year;
        let year_name = commands::get_year_name(&year);
        let year_path = project_directory.join("src").join(&year_name);
        let last_day_num = commands::get_last_day(&year_path)?;

        let test_argument = format!("{}::{}", year_name, commands::get_day_name(last_day_num));

        let mut run_tests = process::Command::new("cargo");
        run_tests.current_dir(project_directory);
        run_tests.arg("test");

        if self.release {
            run_tests.arg("-r");
        }

        if self.nocapture {
            run_tests.arg("--");
            run_tests.arg("--nocapture");
        }

        run_tests.arg(test_argument).status()?;

        Ok(())
    }
}

use anyhow::Result;
use clap::{Args, CommandFactory};
use clap_complete::{generate, Shell};
use std::io;

/// Generate completion file for a given shell.
/// Save the output of this command to your shell to get auto completion
#[derive(Args, Debug)]
#[command()]
pub struct Command {
    #[arg(value_enum)]
    shell: Shell,
}

impl Command {
    pub fn run(&self) -> Result<()> {
        let mut cmd = crate::commands::Command::command();
        let bin_name = cmd.get_name().to_string();

        generate(self.shell, &mut cmd, bin_name, &mut io::stdout());

        Ok(())
    }
}

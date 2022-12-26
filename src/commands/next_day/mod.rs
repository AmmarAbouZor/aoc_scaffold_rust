use anyhow::Result;
use clap::Args;

#[derive(Args, Debug)]
#[command()]
pub struct Command {}

impl Command {
    pub fn run(&self) -> Result<()> {
        todo!();
    }
}

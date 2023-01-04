use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

const APP_NAME: &str = "aoc_scaff";
const CONFIG_NAME: &str = "config";

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Config {
    pub year: String,
}

pub fn get_path() -> Result<PathBuf> {
    confy::get_configuration_file_path(APP_NAME, CONFIG_NAME).context("can't get config path")
}

pub fn load() -> Result<Config> {
    confy::load(APP_NAME, CONFIG_NAME).context("can't load config")
}

pub fn save(config: Config) -> Result<()> {
    confy::store(APP_NAME, CONFIG_NAME, config).context("can't save config")
}

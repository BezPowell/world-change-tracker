use super::source::Sources;
use serde::{Deserialize, Serialize};
use std::{error::Error, fs};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub start_year: u32,
    pub end_year: u32,
    pub output_path: String,
    pub sources: Sources,
}

impl Config {
    pub fn load(path: &str) -> Result<Config, Box<dyn Error>> {
        let file = fs::read_to_string(path)?;
        let config = serde_json::from_str(&file)?;

        Ok(config)
    }
}

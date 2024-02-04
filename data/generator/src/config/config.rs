use super::schema::Schema;
use crate::data::Table;
use serde::Deserialize;
use std::{collections::HashMap, error::Error, fs::File, io::BufReader, path::PathBuf};

#[derive(Debug, Deserialize)]
pub struct Config {
    start: u32,
    end: u32,
    output: PathBuf,
    #[serde(default)]
    directory: PathBuf,
    sources: Vec<Schema>,
}

impl Config {
    pub fn load(path: &str) -> Result<Self, Box<dyn Error>> {
        // Open the file in read-only mode with buffer.
        let path = PathBuf::from(path);
        let file = File::open(path.clone())?;
        let reader = BufReader::new(file);

        // Read the JSON contents of the file.
        let mut source: Config = serde_json::from_reader(reader)?;
        source.directory = path.parent().unwrap().to_path_buf();

        Ok(source)
    }

    pub fn data(&self) -> Result<HashMap<String, Table>, Box<dyn Error>> {
        let mut tables = HashMap::with_capacity(self.sources.len());

        for schema in &self.sources {
            let mut path = self.directory.clone();
            path.push(schema.path());

            let table = Table::load(&path, schema.columns())?;
            tables.insert(schema.name().to_owned(), table);
        }

        Ok(tables)
    }

    pub fn start(&self) -> u32 {
        self.start
    }

    pub fn end(&self) -> u32 {
        self.end
    }

    pub fn output(&self) -> &PathBuf {
        &self.output
    }
}

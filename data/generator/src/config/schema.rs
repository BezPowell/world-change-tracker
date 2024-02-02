use super::column::Column;
use serde::Deserialize;
use std::path::PathBuf;

/// Schema describing a single data source.
#[derive(Debug, Deserialize)]
pub struct Schema {
    /// Name of the data source.
    name: String,
    /// Path to the data source as a csv file.
    path: PathBuf,
    /// Metadata describing expected columns and their format.
    columns: Vec<Column>,
}

impl Schema {
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn columns(&self) -> &[Column] {
        self.columns.as_ref()
    }
}

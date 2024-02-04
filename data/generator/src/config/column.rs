use super::data_type::DataType;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Column {
    name: String,
    data_type: DataType,
}

impl Column {
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn data_type(&self) -> &DataType {
        &self.data_type
    }
}

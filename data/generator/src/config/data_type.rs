use std::error::Error;

use serde::Deserialize;

use crate::data::Value;

#[derive(Debug, Deserialize)]
pub enum DataType {
    Float,
    Integer,
}

impl DataType {
    pub fn parse(&self, string: &str) -> Result<Value, Box<dyn Error>> {
        match self {
            DataType::Float => Ok(Value::Float(string.parse()?)),
            DataType::Integer => Ok(Value::Integer(string.parse()?)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{config::data_type::DataType, data::Value};

    #[test]
    fn parse() {
        assert_eq!(DataType::Integer.parse("10").unwrap(), Value::Integer(10));
        assert_eq!(
            DataType::Float.parse("243.15").unwrap(),
            Value::Float(243.15)
        );
    }

    #[test]
    #[should_panic]
    fn parse_invalid() {
        DataType::Integer.parse("NaN").unwrap();
    }
}

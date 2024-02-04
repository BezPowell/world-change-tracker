use super::{point::Point, Value};
use crate::config::column::Column;
use close_enough::diff::Diff;
use std::{collections::HashMap, error::Error, path::PathBuf};

/// Represents a single source; a table of data.
#[derive(Debug)]
pub struct Table {
    headers: Vec<String>,
    rows: HashMap<u32, Vec<Option<Value>>>,
}

impl Table {
    pub fn load(path: &PathBuf, columns: &[Column]) -> Result<Table, Box<dyn Error>> {
        let headers = columns.iter().map(|x| x.name().to_owned()).collect();
        let mut rows = HashMap::new();
        let mut rdr = csv::Reader::from_path(path)?;
        for result in rdr.records() {
            let record = result?;
            let mut values = Vec::new();

            // Parse columns
            let key: u32 = record.get(0).unwrap().parse()?;
            for (index, column) in columns.iter().enumerate() {
                let value = match record.get(index + 1) {
                    Some(value) => {
                        if value.is_empty() {
                            None
                        } else {
                            Some(column.data_type().parse(value)?)
                        }
                    }
                    None => None,
                };

                values.push(value);
            }

            // Insert row
            rows.insert(key, values);
        }

        Ok(Table { headers, rows })
    }

    pub fn get(&self, row: &u32, column: &str) -> &Option<Value> {
        match self.headers.iter().position(|x| x == column) {
            Some(index) => match self.rows.get(row) {
                Some(row) => match row.get(index) {
                    Some(value) => value,
                    None => &None,
                },
                None => &None,
            },
            None => &None,
        }
    }

    pub fn get_closest_match(&self, year: &u32, column: &str) -> Option<Point> {
        // Get vec of keys
        let mut keys: Vec<&u32> = self.rows.keys().collect();

        // Sort by date first, then by diff
        keys.sort();
        keys.sort_by(|a, b| a.diff(&year).cmp(&b.diff(&year)));

        // Iterate through keys and return the first that has a value
        // for column.
        for key in keys {
            if let Some(value) = self.get(key, column) {
                return Some(Point::new(*key, *value));
            }
        }

        None
    }

    pub fn headers(&self) -> &[String] {
        self.headers.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::Table;
    use crate::{
        config::config::Config,
        data::{point::Point, Value},
    };
    use std::collections::HashMap;

    fn test_data() -> HashMap<String, Table> {
        let config = Config::load("test/config.json").unwrap();
        let tables = config.data().unwrap();

        tables
    }

    #[test]
    fn get() {
        let tables = test_data();
        let integer = tables.get("integer").unwrap();

        assert_eq!(integer.get(&1950, "integer").unwrap(), Value::Integer(0));

        assert_eq!(integer.get(&1951, "other_value"), &None);

        assert_eq!(integer.get(&6, "integer"), &None);
    }

    #[test]
    fn get_closest() {
        let tables = test_data();
        let integer = tables.get("integer").unwrap();
        let float = tables.get("float").unwrap();

        assert_eq!(
            integer.get_closest_match(&1952, "integer").unwrap(),
            Point::new(1951, Value::Integer(1))
        );

        assert_eq!(
            float.get_closest_match(&1952, "float").unwrap(),
            Point::new(1953, Value::Float(3.0))
        );
    }
}

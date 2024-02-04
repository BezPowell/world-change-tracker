use serde::Serialize;

use super::Value;

/// Represents a single data point and the key it is associated with.
#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct Point {
    key: u32,
    value: Value,
}

impl Point {
    pub fn new(key: u32, value: Value) -> Point {
        Point { key, value }
    }

    pub fn value(&self) -> Value {
        self.value
    }
}

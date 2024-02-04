use std::ops::{Div, Sub};

use serde::Serialize;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Value {
    Float(f64),
    Integer(u64),
}

impl Sub for Value {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match self {
            Value::Float(lhs) => match rhs {
                Value::Float(rhs) => Value::Float(lhs - rhs),
                Value::Integer(rhs) => Value::Float(lhs - rhs as f64),
            },
            Value::Integer(lhs) => match rhs {
                Value::Float(rhs) => Value::Float(lhs as f64 - rhs),
                Value::Integer(rhs) => Value::Integer(lhs - rhs),
            },
        }
    }
}

impl Div for Value {
    type Output = f64;

    fn div(self, rhs: Self) -> Self::Output {
        let lhs: f64 = self.into();
        let rhs: f64 = rhs.into();

        lhs / rhs
    }
}

impl Into<f64> for Value {
    fn into(self) -> f64 {
        match self {
            Value::Float(value) => value,
            Value::Integer(value) => value as f64,
        }
    }
}

impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Value::Float(value) => serializer.serialize_f64(*value),
            Value::Integer(value) => serializer.serialize_u64(*value),
        }
    }
}

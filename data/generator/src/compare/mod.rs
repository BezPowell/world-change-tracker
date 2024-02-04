use serde::Serialize;

use crate::data::{Point, Value};

#[derive(Debug, Serialize)]
pub struct Compare {
    old: Point,
    new: Point,
    absolute: Value,
    percent: f64,
}

impl Compare {
    pub fn compare(old: &Point, new: &Point) -> Compare {
        let absolute = new.value() - old.value();
        let percent = (absolute / old.value()) * 100.0;

        Compare {
            old: old.clone(),
            new: new.clone(),
            absolute,
            percent,
        }
    }
}

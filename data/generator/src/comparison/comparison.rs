use super::{change::Change, data_point::DataPoint};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Comparison<T>
where
    T: std::convert::Into<f64> + std::cmp::PartialOrd + Copy,
{
    old: DataPoint<T>,
    new: DataPoint<T>,
    percent: f64,
    change: String,
}

impl<T> Comparison<T>
where
    T: std::convert::Into<f64> + std::cmp::PartialOrd + Clone + Copy,
{
    pub fn compare(old: &DataPoint<T>, new: &DataPoint<T>, change: Change) -> Comparison<T> {
        let change = change.string(old, new);
        let diff = new.value() - old.value();
        let percent = (diff / old.value()) * 100.0;

        Comparison {
            old: old.clone(),
            new: new.clone(),
            percent,
            change,
        }
    }
}

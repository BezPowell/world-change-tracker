use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DataPoint<T>
where
    T: std::convert::Into<f64> + std::cmp::PartialOrd + Copy,
{
    pub(crate) value: T,
    pub(crate) year: u32,
}

impl<T> DataPoint<T>
where
    T: std::convert::Into<f64> + std::cmp::PartialOrd + Copy,
{
    pub fn value(&self) -> f64 {
        self.value.into()
    }
}

impl<T> PartialEq for DataPoint<T>
where
    T: std::convert::Into<f64> + std::cmp::PartialOrd + Copy,
{
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T> PartialOrd for DataPoint<T>
where
    T: std::convert::Into<f64> + std::cmp::PartialOrd + Copy,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

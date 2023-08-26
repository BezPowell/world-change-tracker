use std::error::Error;

use serde::Deserialize;

use crate::comparison::data_point::DataPoint;

pub trait DataType<T, E>
where
    T: std::convert::Into<f64> + std::cmp::PartialOrd + Copy,
{
    fn data(&self, variant: E) -> DataPoint<T>;

    fn deserialize(path: &str, variant: E) -> Result<Vec<DataPoint<T>>, Box<dyn Error>>
    where
        Self: Sized + for<'a> Deserialize<'a>,
        E: Copy
    {
        let mut items = Vec::new();
        let mut reader = csv::Reader::from_path(path)?;

        for row in reader.deserialize() {
            let item: Self = row?;
            items.push(item.data(variant));
        }

        Ok(items)
    }
}

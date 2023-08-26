use crate::comparison::{change::Change, comparison::Comparison, data_point::DataPoint};
use std::collections::HashMap;

use super::traits::CloseMatch;

pub struct DataSource<T>
where
    T: std::convert::Into<f64> + std::cmp::PartialOrd + Copy,
{
    change_type: Change,
    last: u32,
    points: HashMap<u32, DataPoint<T>>,
}

impl<T> DataSource<T>
where
    T: std::convert::Into<f64> + std::cmp::PartialOrd + Copy,
{
    pub fn new(points: Vec<DataPoint<T>>, change_type: Change) -> DataSource<T> {
        let last = points.last().expect("Data source is empty").year;
        let mut map = HashMap::with_capacity(points.len());

        for point in points {
            map.insert(point.year, point);
        }

        DataSource {
            change_type,
            last: last,
            points: map,
        }
    }

    pub fn compare(&self, year: u32) -> Comparison<T> {
        let new = self.points.get(&self.last).unwrap();

        let old = self
            .points
            .get_closest_match(&year)
            .expect(&format!("Unable to get data for {}", year));

        Comparison::compare(old, new, self.change_type)
    }
}

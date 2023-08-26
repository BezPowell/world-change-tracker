use super::data_type::DataType;
use crate::comparison::data_point::DataPoint;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy)]
pub enum TemperatureVariant {
    Temperature,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Temperature {
    pub year: u32,
    pub value: f32,
}

impl DataType<f32, TemperatureVariant> for Temperature {
    fn data(&self, variant: TemperatureVariant) -> crate::comparison::data_point::DataPoint<f32> {
        DataPoint {
            value: self.value,
            year: self.year,
        }
    }
}

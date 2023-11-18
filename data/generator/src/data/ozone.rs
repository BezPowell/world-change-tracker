use super::data_type::DataType;
use crate::comparison::data_point::DataPoint;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy)]
pub enum OzoneVariant {
    Ozone,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ozone {
    pub year: u32,
    pub area: f32,
}

impl DataType<f32, OzoneVariant> for Ozone {
    fn data(&self, variant: OzoneVariant) -> crate::comparison::data_point::DataPoint<f32> {
        DataPoint {
            value: self.area,
            year: self.year,
        }
    }
}

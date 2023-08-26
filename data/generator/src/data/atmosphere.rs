use super::data_type::DataType;
use crate::comparison::data_point::DataPoint;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy)]
pub enum AtmosphereVariant {
    Carbon,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Atmosphere {
    pub year: u32,
    pub carbon: f32,
}

impl DataType<f32, AtmosphereVariant> for Atmosphere {
    fn data(&self, variant: AtmosphereVariant) -> crate::comparison::data_point::DataPoint<f32> {
        let value = match variant {
            AtmosphereVariant::Carbon => self.carbon,
        };

        DataPoint {
            value: value,
            year: self.year,
        }
    }
}

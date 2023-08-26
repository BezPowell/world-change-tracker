use super::data_type::DataType;
use crate::comparison::data_point::DataPoint;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy)]
pub enum LivingPlanetVariant {
    LivingPlanet,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LivingPlanet {
    pub year: u32,
    pub value: f32,
}

impl DataType<f32, LivingPlanetVariant> for LivingPlanet {
    fn data(&self, variant: LivingPlanetVariant) -> crate::comparison::data_point::DataPoint<f32> {
        DataPoint {
            value: self.value,
            year: self.year,
        }
    }
}

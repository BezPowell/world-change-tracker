use super::data_type::DataType;
use crate::comparison::data_point::DataPoint;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy)]
pub enum RedListVariant {
    RedListIndex,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RedListIndex {
    pub year: u32,
    pub rli: f32,
}

impl DataType<f32, RedListVariant> for RedListIndex {
    fn data(&self, variant: RedListVariant) -> crate::comparison::data_point::DataPoint<f32> {
        DataPoint {
            value: self.rli,
            year: self.year,
        }
    }
}

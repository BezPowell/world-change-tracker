use super::data_type::DataType;
use crate::comparison::data_point::DataPoint;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy)]
pub enum PopulationVariant {
    Population,
    LifeExpectancy,
    InfantMortality,
    MedianAge,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Population {
    pub year: u32,
    pub population: f64,
    pub life_expectancy: f32,
    pub infant_mortality: f32,
    pub median_age: f32,
}

impl DataType<f64, PopulationVariant> for Population {
    fn data(&self, variant: PopulationVariant) -> crate::comparison::data_point::DataPoint<f64> {
        let value = match variant {
            PopulationVariant::Population => self.population,
            PopulationVariant::LifeExpectancy => self.life_expectancy.into(),
            PopulationVariant::InfantMortality => self.infant_mortality.into(),
            PopulationVariant::MedianAge => self.median_age.into(),
        };

        DataPoint {
            value: value,
            year: self.year,
        }
    }
}

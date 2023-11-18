use super::comparison::Comparison;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Year {
    year: u32,
    population: Comparison<f64>,
    life_expectancy: Comparison<f64>,
    infant_mortality: Comparison<f64>,
    median_age: Comparison<f64>,
    carbon: Comparison<f32>,
    living_planet: Comparison<f32>,
    temperature: Comparison<f32>,
    red_list_index: Comparison<f32>,
    ozone: Comparison<f32>,
}

impl Year {
    pub fn new(
        year: u32,
        population: Comparison<f64>,
        life_expectancy: Comparison<f64>,
        infant_mortality: Comparison<f64>,
        median_age: Comparison<f64>,
        carbon: Comparison<f32>,
        living_planet: Comparison<f32>,
        temperature: Comparison<f32>,
        red_list_index: Comparison<f32>,
        ozone: Comparison<f32>,
    ) -> Self {
        Self {
            year,
            population,
            life_expectancy,
            infant_mortality,
            median_age,
            carbon,
            living_planet,
            temperature,
            red_list_index,
            ozone,
        }
    }
}

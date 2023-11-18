use crate::{
    comparison::{change::Change, year::Year},
    config::Config,
    data::{
        atmosphere::{Atmosphere, AtmosphereVariant},
        data_source::DataSource,
        data_type::DataType,
        living_planet::{LivingPlanet, LivingPlanetVariant},
        ozone::Ozone,
        population::{Population, PopulationVariant},
        red_list_index::{RedListIndex, RedListVariant},
        temperature::{Temperature, TemperatureVariant},
    },
    frontmatter::Frontmatter,
};
use std::error::Error;

/// The main application
pub struct App {
    config: Config,
    population: DataSource<f64>,
    life_expectancy: DataSource<f64>,
    infant_mortality: DataSource<f64>,
    median_age: DataSource<f64>,
    carbon: DataSource<f32>,
    temperature: DataSource<f32>,
    living_planet: DataSource<f32>,
    red_list_index: DataSource<f32>,
    ozone: DataSource<f32>,
}

impl App {
    pub fn new(config: &str) -> Result<App, Box<dyn Error>> {
        let config = Config::load(config)?;
        let sources = &config.sources;

        // Build data
        // Population
        let population = DataSource::new(
            Population::deserialize(&sources.population.path, PopulationVariant::Population)?,
            sources.population.change,
        );
        let life_expectancy = DataSource::new(
            Population::deserialize(&sources.population.path, PopulationVariant::LifeExpectancy)?,
            Change::Increased,
        );
        let infant_mortality = DataSource::new(
            Population::deserialize(&sources.population.path, PopulationVariant::InfantMortality)?,
            Change::Risen,
        );
        let median_age = DataSource::new(
            Population::deserialize(&sources.population.path, PopulationVariant::MedianAge)?,
            Change::Increased,
        );

        let carbon = DataSource::new(
            Atmosphere::deserialize(&sources.atmosphere.path, AtmosphereVariant::Carbon)?,
            sources.atmosphere.change,
        );
        let temperature = DataSource::new(
            Temperature::deserialize(&sources.temperature.path, TemperatureVariant::Temperature)?,
            sources.temperature.change,
        );

        let living_planet = DataSource::new(
            LivingPlanet::deserialize(
                &sources.living_planet.path,
                LivingPlanetVariant::LivingPlanet,
            )?,
            sources.living_planet.change,
        );
        let red_list_index = DataSource::new(
            RedListIndex::deserialize(&sources.red_list_index.path, RedListVariant::RedListIndex)?,
            sources.red_list_index.change,
        );

        let ozone = DataSource::new(
            Ozone::deserialize(&sources.ozone.path, crate::data::ozone::OzoneVariant::Ozone)?,
            sources.ozone.change,
        );

        Ok(App {
            config,
            population,
            life_expectancy,
            infant_mortality,
            median_age,
            carbon,
            temperature,
            living_planet,
            red_list_index,
            ozone,
        })
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn build_templates(&self) -> Vec<Frontmatter> {
        let years: Vec<u32> = (self.config.start_year..=self.config.end_year).collect();
        let mut templates = Vec::with_capacity(years.len());

        for year in years {
            // Population
            let population = self.population.compare(year);
            let life_expectancy = self.life_expectancy.compare(year);
            let infant_mortality = self.infant_mortality.compare(year);
            let median_age = self.median_age.compare(year);

            let carbon = self.carbon.compare(year);
            let temperature = self.temperature.compare(year);
            let ozone = self.ozone.compare(year);

            let living_planet = self.living_planet.compare(year);
            let red_list_index = self.red_list_index.compare(year);

            let extra = Year::new(
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
            );

            templates.push(Frontmatter::new(year, extra))
        }

        templates
    }
}

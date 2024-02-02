use crate::{compare::Compare, config::config::Config};
use serde::Serialize;
use std::{
    collections::HashMap,
    error::Error,
    fs::{self, File},
    io::Write,
};

#[derive(Debug, Serialize)]
struct Page {
    title: String,
    extra: Extra,
}

#[derive(Debug, Serialize)]
struct Extra {
    year: u32,
}

pub fn write_data(config: &Config) -> Result<(), Box<dyn Error>> {
    let tables = config.data()?;
    let keys: Vec<u32> = (config.start()..=config.end()).collect();

    if let Some(end) = keys.last() {
        for key in keys.iter() {
            // Try create directory
            let mut path = config.output().clone();
            path.push(key.to_string());
            fs::create_dir_all(&path)?;

            // Compare data
            let mut out_data = HashMap::new();
            for (_source, table) in &tables {
                for field in table.headers() {
                    let value = table.get_closest_match(key, field).unwrap();
                    let end = table.get_closest_match(end, field).unwrap();
                    let compare = Compare::compare(&value, &end);

                    out_data.insert(field, compare);
                }
            }

            let mut out_path = path.clone();
            out_path.push("data.json");

            let json = serde_json::to_string(&out_data)?;
            let mut out_file = File::create(&out_path)?;
            write!(out_file, "{}", json)?;
        }
    }

    Ok(())
}

pub fn write_pages(config: &Config) -> Result<(), Box<dyn Error>> {
    let keys: Vec<u32> = (config.start()..=config.end()).collect();

    for key in keys.iter() {
        // Try create directory
        let mut path = config.output().clone();
        path.push(key.to_string());
        fs::create_dir_all(&path)?;

        // Generate content
        let page = Page {
            title: key.to_string(),
            extra: Extra { year: *key },
        };
        let toml = toml::to_string(&page)?;

        // Write to file
        let mut path = path.clone();
        path.push("index.md");
        let mut out_file = File::create(&path)?;
        write!(out_file, "+++\n{}+++", toml)?;
    }

    Ok(())
}

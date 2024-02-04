use crate::{
    compare::Compare,
    config::config::Config,
    output::{Index, Page},
};
use std::{
    collections::HashMap,
    error::Error,
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

pub fn write_data(config: &Config) -> Result<(), Box<dyn Error>> {
    let tables = config.data()?;
    let keys: Vec<u32> = (config.start()..config.end()).collect();
    let end = &config.end();

    for key in keys.iter() {
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

        let mut dir = config.output().clone();
        dir.push(key.to_string());
        let json = serde_json::to_string(&out_data)?;
        write_file(&dir, "data.json", &json)?;
    }

    Ok(())
}

pub fn write_pages(config: &Config) -> Result<(), Box<dyn Error>> {
    let keys: Vec<u32> = (config.start()..config.end()).collect();

    // Generate Index
    let index = Index::new("Years", "year.html", 20).serialize()?;
    write_file(config.output(), "_index.md", &index)?;

    // The individual years
    for key in keys.iter() {
        // Generate content
        let page = Page::new(&key.to_string(), *key).serialize()?;

        // Write to file
        let mut dir = config.output().clone();
        dir.push(key.to_string());
        write_file(&dir, "index.md", &page)?;
    }

    Ok(())
}

fn write_file(directory: &PathBuf, filename: &str, content: &str) -> Result<(), std::io::Error> {
    // Try create directory
    let mut path = directory.clone();
    fs::create_dir_all(&path)?;

    // Write file
    path.push(filename);
    let mut out_file = File::create(&path)?;
    write!(out_file, "{}", content)?;

    Ok(())
}

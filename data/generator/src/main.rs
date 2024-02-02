use generics::{write_data, write_pages, Config};
use std::fs;

fn main() {
    let config = Config::load("config.json").unwrap();

    // Clear existing content if exists
    let _ = fs::remove_dir_all(config.output());

    // Write new data.
    write_data(&config).expect("Unable to write data.");
    write_pages(&config).expect("Unable to write pages.");
}

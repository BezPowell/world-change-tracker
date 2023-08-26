use crate::comparison::year::Year;
use serde::Serialize;
use std::io::Write;
use std::{
    error::Error,
    fs::{self, File},
    path::Path,
};

#[derive(Debug, Serialize)]
pub struct Frontmatter {
    title: String,
    extra: Year,
}

impl Frontmatter {
    pub fn new(year: u32, extra: Year) -> Self {
        Self {
            title: year.to_string(),
            extra,
        }
    }

    pub fn write(&self, path: &str) -> Result<(), Box<dyn Error>> {
        // Create template and parent directories
        let path_str = format!("{}{}.md", path, self.title);
        let path = Path::new(&path_str);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let mut output = File::create(path)?;

        // Build template
        let contents = format!("+++\n{}\n+++", toml::to_string(&self)?);

        // Write template contents to file.
        write!(output, "{}", contents)?;

        Ok(())
    }
}

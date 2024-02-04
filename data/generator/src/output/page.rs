use serde::Serialize;
use toml::ser::Error;

#[derive(Debug, Serialize)]
pub struct Page {
    title: String,
    extra: Extra,
}

#[derive(Debug, Serialize)]
struct Extra {
    year: u32,
}

impl Page {
    pub fn new(title: &str, year: u32) -> Page {
        Page {
            title: title.to_string(),
            extra: Extra { year: year },
        }
    }

    pub fn serialize(self) -> Result<String, Error> {
        let content = toml::to_string(&self)?;

        Ok(format!("+++\n{}+++", content))
    }
}

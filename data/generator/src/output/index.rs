use serde::Serialize;
use toml::ser::Error;

#[derive(Debug, Serialize)]
pub struct Index {
    title: String,
    page_template: String,
    paginate_by: u32,
    sort_by: String,
    paginate_reversed: bool,
}

impl Index {
    pub fn new(title: &str, page_template: &str, paginate_by: u32) -> Index {
        Index {
            title: title.to_string(),
            page_template: page_template.to_string(),
            paginate_by,
            sort_by: "title".to_string(),
            paginate_reversed: true,
        }
    }

    pub fn serialize(&self) -> Result<String, Error> {
        let content = toml::to_string(&self)?;

        Ok(format!("+++\n{}+++", content))
    }
}

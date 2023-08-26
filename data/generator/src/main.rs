mod app;
mod comparison;
mod config;
mod data;
mod frontmatter;

use app::App;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let app = App::new("config.json")?;
    let config = app.config();
    let templates = app.build_templates();

    for template in templates {
        template.write(&config.output_path)?;
    }

    Ok(())
}

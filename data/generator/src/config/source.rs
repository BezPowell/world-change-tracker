use serde::{Deserialize, Serialize};

use crate::comparison::change::Change;

#[derive(Debug, Serialize, Deserialize)]
pub struct Sources {
    pub population: Source,
    pub atmosphere: Source,
    pub temperature: Source,
    pub living_planet: Source,
    pub red_list_index: Source,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Source {
    pub path: String,
    pub change: Change,
}

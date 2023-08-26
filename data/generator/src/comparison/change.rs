use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum Change {
    Increased,
    Grown,
    Risen,
}

impl Change {
    pub fn string<T>(&self, old: &T, new: &T) -> String
    where
        T: PartialOrd,
    {
        let result = match new > old {
            true => match self {
                Change::Increased => "increased",
                Change::Grown => "grown",
                Change::Risen => "risen",
            },
            false => match self {
                Change::Increased => "decreased",
                Change::Grown => "shrunk",
                Change::Risen => "fallen",
            },
        };

        result.to_string()
    }
}

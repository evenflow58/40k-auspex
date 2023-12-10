use serde::{Deserialize, Serialize};

// Create a struct named DisplayUnit that has the following fields
// Name: String
// and that's it
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DisplayUnit {
    pub name: String,
    pub enhancement: Option<String>,
}

impl DisplayUnit {
    pub fn new(name: String) -> Self {
        DisplayUnit {
            name,
            enhancement: None,
        }
    }
}
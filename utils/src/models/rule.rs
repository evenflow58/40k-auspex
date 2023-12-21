use serde::{Deserialize, Serialize};

// Create a struct named Army that contains a list of Units
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Rule {
    pub name: String,
    pub tags: Vec<String>,
}

impl Rule {
    pub fn new(name: String, tags: Vec<String>) -> Self {
        Rule {
            name,
            tags,
        }
    }

}
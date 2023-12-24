use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Enhancement {
    pub name: String,
    pub tags: Vec<String>,
}

impl Enhancement {
    pub fn new(
        name: String,
        tags: &Vec<String>
    ) -> Self {
        Enhancement {
            name,
            tags: tags.to_vec(),
        }
    }
}
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Ability {
    name: String,
    tags: Option<Vec<String>>
}

impl Ability {
    pub fn new(
        name: String,
        tags: Option<Vec<String>>
    ) -> Self {
        Ability {
            name,
            tags,
        }
    }
}
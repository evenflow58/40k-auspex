use serde::{Deserialize, Serialize};

use crate::enums::ability_type::AbilityType;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Ability {
    name: String,
    tags: Option<Vec<String>>,
    text: Option<String>,
    ability_type: AbilityType,
}

impl Ability {
    pub fn new(
        name: String,
        tags: Option<Vec<String>>,
        text: Option<String>,
        ability_type: AbilityType,
    ) -> Self {
        Ability {
            name,
            tags,
            text,
            ability_type,
        }
    }
}
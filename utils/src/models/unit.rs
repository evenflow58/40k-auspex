use crate::models::{ability::Ability, composition::Composition, weapon::Weapon};
use serde::{Deserialize, Serialize};

// Create a struct named Unit that has the following fields
// Name: String
// Abilities: Ability
// and that's it
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Unit {
    name: String,
    abilities: Vec<Ability>
}

impl Unit {
    pub fn new(name: String,
        abilities: Vec<Ability>
    ) -> Self {
        Unit {
            name,
            abilities
        }
    }
}
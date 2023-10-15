use crate::models::{unit::Unit, faction::Faction};
use serde::{Deserialize, Serialize};

// Create a struct named Army that contains a list of Units
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Army {
    pub name: String,
    pub units: Vec<Unit>,
    pub factions: Vec<Faction>,
}

impl Army {
    pub fn new(name: String, units: Vec<Unit>, factions: Vec<Faction>) -> Self {
        Army {
            name,
            units,
            factions,
        }
    }

}
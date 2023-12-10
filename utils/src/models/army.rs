use crate::models::{unit::Unit};
use serde::{Deserialize, Serialize};

// Create a struct named Army that contains a list of Units
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Army {
    pub name: String,
    pub units: Vec<Unit>,
    pub factions: Vec<String>,
}

impl Army {
    pub fn new(name: String, units: Vec<Unit>, factions: Vec<String>) -> Self {
        Army {
            name,
            units,
            factions,
        }
    }

}
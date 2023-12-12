use crate::models::{unit::Unit, enhancment::Enhancement};
use serde::{Deserialize, Serialize};

// Create a struct named Army that contains a list of Units
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Army {
    pub name: String,
    pub units: Vec<Unit>,
    pub factions: Vec<String>,
    pub enhancements: Vec<Enhancement>,
}

impl Army {
    pub fn new(name: String, units: Vec<Unit>, factions: Vec<String>, enhancements: Vec<Enhancement>) -> Self {
        Army {
            name,
            units,
            factions,
            enhancements,
        }
    }

}
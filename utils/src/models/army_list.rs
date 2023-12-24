use crate::models::unit::Unit;
use serde::{Deserialize, Serialize};

// Create a struct named Army that contains a list of Units
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ArmyList {
    pub name: String,
    pub faction: String,
    pub units: Vec<Unit>,
}

impl ArmyList {
    pub fn new(name: String, faction: String, units: Vec<Unit>) -> Self {
        ArmyList {
            name,
            units,
            faction,
        }
    }

}
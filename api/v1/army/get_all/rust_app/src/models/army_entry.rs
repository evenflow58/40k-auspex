use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct ArmyEntry {
    pub name: String,
    pub factions: Vec<String>,
}
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Faction {
    pub name: String,
    pub tag: String,
}

impl Faction {
    pub fn new(name: String, tag: String) -> Self {
        Faction { name, tag }
    }
}
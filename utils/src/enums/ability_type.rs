use core::fmt;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum AbilityType {
    Core,
    Faction,
    Unique,
    Wargear,
}

impl fmt::Display for AbilityType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AbilityType::Core => write!(f, "core"),
            AbilityType::Faction => write!(f, "faction"),
            AbilityType::Unique => write!(f, "unique"),
            AbilityType::Wargear => write!(f, "wargear"),
        }
    }
}
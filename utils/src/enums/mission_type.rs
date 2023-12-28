use core::fmt;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum MissionType {
    Fixed,
    Tactical,
}

impl fmt::Display for MissionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MissionType::Fixed => write!(f, "fixed"),
            MissionType::Tactical => write!(f, "tactical"),
        }
    }
}
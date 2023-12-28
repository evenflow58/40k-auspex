use core::fmt;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum GameSize {
    Incursion,
    StrikeForce,
    Onslaught,
}

impl fmt::Display for GameSize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GameSize::Incursion => write!(f, "incursion"),
            GameSize::StrikeForce => write!(f, "strike_force"),
            GameSize::Onslaught => write!(f, "onslaught"),
        }
    }
}
use core::fmt;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Restriction {
    Warlord,
    Epic,
}

impl fmt::Display for Restriction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Restriction::Warlord => write!(f, "warlord"),
            Restriction::Epic => write!(f, "epic"),
        }
    }
}
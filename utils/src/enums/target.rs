use core::fmt;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Target {
    Daemon,
    Infantry
}

impl fmt::Display for Target {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Target::Daemon => write!(f, "daemon"),
            Target::Infantry => write!(f, "infantry"),
        }
    }
}
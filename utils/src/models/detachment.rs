use serde::{Deserialize, Serialize};
use crate::models::rule::Rule;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Detachment {
    pub name: String,
    pub rule: Rule,
}

impl Detachment {
    pub fn new(name: String, rule: Rule) -> Self {
        Detachment { name, rule }
    }
}
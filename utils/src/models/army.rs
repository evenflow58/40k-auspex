use crate::models::{unit::Unit, enhancment::Enhancement, rule::Rule, detachment::Detachment};
use serde::{Deserialize, Serialize};

// Create a struct named Army that contains a list of Units
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Army {
    pub name: String,
    pub army_rule: Rule,
    pub detachments: Vec<Detachment>,
    pub units: Vec<Unit>,
    pub enhancements: Vec<Enhancement>,
}

impl Army {
    pub fn new(name: String, army_rule: Rule, detachments: Vec<Detachment>, units: Vec<Unit>, enhancements: Vec<Enhancement>) -> Self {
        Army {
            name,
            army_rule,
            detachments,
            units,
            enhancements,
        }
    }

}
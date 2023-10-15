use serde::{Deserialize, Serialize};
use crate::enums::target::Target;

// Create a struct named effect that contains a type, target and amount
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Effect {
    effect_type: String,
    target: Option<Target>,
    amount: Option<i32>,
}

impl Effect {
    pub fn new(effect_type: String, target: Option<Target>, amount: Option<i32>) -> Self {
        Effect {
            effect_type,
            target,
            amount,
        }
    }
}
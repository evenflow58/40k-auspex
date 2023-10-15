use serde::{Deserialize, Serialize};

use crate::enums::restriction::Restriction;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Composition {
    amount: i32,
    attach_to: Option<Vec<String>>,
    restrictions: Option<Vec<Restriction>>,
}

impl Composition {
    pub fn new(amount: i32, attach_to: Option<Vec<String>>, restrictions: Option<Vec<Restriction>>) -> Self {
        Composition {
            amount,
            attach_to,
            restrictions,
        }
    }
}
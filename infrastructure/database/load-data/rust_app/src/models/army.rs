use serde::{Deserialize};
use std::collections::HashMap;
use aws_sdk_dynamodb::{types::{AttributeValue}};
use crate::models::unit::Unit;

// Create a struct named Army that contains a list of Units
#[derive(Deserialize, Debug)]
pub struct Army {
    // #[serde(deserialize_with = "get_hash_map")]
    units: Vec<Unit>,
}

impl Army {
    pub fn get_hash_map(&self) -> HashMap<String, AttributeValue> {
        let mut map = HashMap::new();
        map
            .insert("units".to_string(), AttributeValue::L(
                self
                .units
                .iter()
                .map(|unit| AttributeValue::M(unit.get_hash_map()))
                .collect()
            ));
        map
    }
}
use crate::models::unit::Unit;
use aws_sdk_dynamodb::types::AttributeValue;
use serde::Deserialize;
use std::collections::HashMap;

// Create a struct named Army that contains a list of Units
#[derive(Deserialize, Debug)]
pub struct Army {
    pub name: String,
    units: Vec<Unit>,
}

impl Army {
    pub fn get_hash_map(&self) -> HashMap<String, AttributeValue> {
        let mut map = HashMap::new();
        map.insert(
            "units".to_string(),
            AttributeValue::L(
                self.units
                    .iter()
                    .map(|unit| AttributeValue::M(unit.get_hash_map()))
                    .collect(),
            ),
        );
        map
    }
}

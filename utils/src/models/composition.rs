use aws_sdk_dynamodb::types::AttributeValue;
use serde::Deserialize;
use std::collections::HashMap;

use crate::enums::restriction::Restriction;

#[derive(Deserialize, Debug)]
pub struct Composition {
    amount: i32,
    attach_to: Option<Vec<String>>,
    restrictions: Option<Vec<Restriction>>,
}

impl Composition {
    pub fn get_hash_map(&self) -> HashMap<String, AttributeValue> {
        let mut map = HashMap::new();
        map.insert(
            "Amount".to_string(),
            AttributeValue::N(self.amount.to_string()),
        );
        match self.attach_to {
            Some(ref attach_to) => {
                map.insert(
                    "AttachTo".to_string(),
                    AttributeValue::L(
                        attach_to
                            .iter()
                            .map(|tag| AttributeValue::S(tag.to_string().clone()))
                            .collect(),
                    ),
                );
            }
            None => {}
        }
        match self.restrictions {
            Some(ref restrictions) => {
                map.insert(
                    "Restrictions".to_string(),
                    AttributeValue::L(
                        restrictions
                            .iter()
                            .map(|tag| AttributeValue::S(tag.to_string().clone()))
                            .collect(),
                    ),
                );
            }
            None => {}
        }
        map
    }
}

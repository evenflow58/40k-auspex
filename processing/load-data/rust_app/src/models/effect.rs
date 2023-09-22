use aws_sdk_dynamodb::types::AttributeValue;
use serde::Deserialize;
use std::collections::HashMap;
use crate::enums::target::Target;

// Create a struct named effect that contains a type, target and amount
#[derive(Deserialize, Debug)]
pub struct Effect {
    effect_type: String,
    target: Option<Target>,
    amount: Option<i32>,
}

impl Effect {
    pub fn get_hash_map(&self) -> HashMap<String, AttributeValue> {
        let mut map = HashMap::new();
        map.insert(
            "Type".to_string(),
            AttributeValue::S(self.effect_type.clone()),
        );
        match &self.target {
            Some(target) => {
                map.insert("Target".to_string(), AttributeValue::S(target.to_string()));
            }
            None => {}
        };
        match self.amount {
            Some(amount) => {
                map.insert("Amount".to_string(), AttributeValue::N(amount.to_string()));
            }
            None => {}
        };

        map
    }
}

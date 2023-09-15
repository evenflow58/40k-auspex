use aws_sdk_dynamodb::types::AttributeValue;
use serde::Deserialize;
use std::collections::HashMap;

// Create a struct named effect that contains a type, target and amount
#[derive(Deserialize, Debug)]
pub struct Effect {
    effect_type: String,
    target: String,
    amount: i32,
}

impl Effect {
    pub fn get_hash_map(&self) -> HashMap<String, AttributeValue> {
        let mut map = HashMap::new();
        map.insert(
            "Type".to_string(),
            AttributeValue::S(self.effect_type.clone()),
        );
        map.insert("Target".to_string(), AttributeValue::S(self.target.clone()));
        map.insert(
            "Amount".to_string(),
            AttributeValue::N(self.amount.to_string()),
        );
        map
    }
}

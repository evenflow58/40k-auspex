use aws_sdk_dynamodb::types::AttributeValue;
use serde::Deserialize;
use std::collections::HashMap;

// Create a struct named effect that contains a type, target and amount
#[derive(Deserialize, Debug)]
pub struct Effect {
    type: str,
    target: str,
    amount: i32
};

impl Effect {
    pub fun get_hash_map(&self) -> HashMap<String, AttributeValue> {
        let mut map = HashMap::new();
        map.insert("Type".to_string(), AttributeValue::S(self.type.clone()));
        map.insert("Target".to_string(), AttributeValue::S(self.target.clone()));
        map.insert("Amount".to_string(), AttributeValue::N(self.amount));
        map
    }
}
use aws_sdk_dynamodb::types::AttributeValue;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struc Faction {
    name: String,
    tag: String,
}

impl Faction {
    pub fn get_hash_map(&self) -> HashMap<String, AttributeValue> {
        let mut map = HashMap::new();
        map.insert("Name".to_string(), AttributeValue::S(self.name.clone()));
        map.insert("Tag".to_string(), AttributeValue::S(self.name.clone()));
        map
    }
}

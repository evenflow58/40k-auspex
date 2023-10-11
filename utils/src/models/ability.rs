use aws_sdk_dynamodb::types::AttributeValue;
use serde::Deserialize;
use std::collections::HashMap;

use crate::enums::ability_type::AbilityType;

#[derive(Deserialize, Debug)]
pub struct Ability {
    name: String,
    tags: Option<Vec<String>>,
    text: Option<String>,
    ability_type: AbilityType,
}

impl Ability {
    pub fn get_hash_map(&self) -> HashMap<String, AttributeValue> {
        let mut map = HashMap::new();
        map.insert("Name".to_string(), AttributeValue::S(self.name.clone()));
        match &self.tags {
            Some(tags) => {
                map.insert(
                    "Tags".to_string(),
                    AttributeValue::L(
                        tags.iter()
                            .map(|tag| AttributeValue::S(tag.clone()))
                            .collect(),
                    ),
                );
            }
            None => {}
        };
        match &self.text {
            Some(text) => {
                map.insert("Text".to_string(), AttributeValue::S(text.clone()));
            }
            None => {}
        };
        map.insert(
            "Type".to_string(),
            AttributeValue::S(self.ability_type.to_string()),
        );
        map
    }
}

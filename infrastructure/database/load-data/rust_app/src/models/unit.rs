use serde::{Deserialize};
use std::collections::HashMap;
use aws_sdk_dynamodb::{types::{AttributeValue}};

// Create a struct named Unit that has the following fields
// Name: String
// Movement: i32
// Toughness: i32
// Save: i32
// WeaponSkill: i32
// Leadership: i32
// ObjectiveControl: i32
// and that's it
#[derive(Deserialize, Debug)]
pub struct Unit { 
    Name: String,
    Movement: i32,
    Toughness: i32,
    Save: i32,
    WeaponSkill: i32,
    Leadership: i32,
    ObjectiveControl: i32
}

impl Unit {
    pub fn get_hash_map(&self) -> HashMap<String, AttributeValue> {
        let mut map = HashMap::new();
        map.insert("Name".to_string(), AttributeValue::S(self.Name.clone()));
        map.insert("Movement".to_string(), AttributeValue::N(self.Movement.to_string()));
        map.insert("Toughness".to_string(), AttributeValue::N(self.Toughness.to_string()));
        map.insert("Save".to_string(), AttributeValue::N(self.Save.to_string()));
        map.insert("WeaponSkill".to_string(), AttributeValue::N(self.WeaponSkill.to_string()));
        map.insert("Leadership".to_string(), AttributeValue::N(self.Leadership.to_string()));
        map.insert("ObjectiveControl".to_string(), AttributeValue::N(self.ObjectiveControl.to_string()));
        map
    }
}
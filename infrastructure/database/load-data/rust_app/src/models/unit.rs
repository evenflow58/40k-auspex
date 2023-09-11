use serde::{Deserialize};
use std::collections::HashMap;
// use aws_sdk_dynamodb::{types::{AttributeValue}};

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
    name: String,
    movement: i32,
    toughness: i32,
    save: i32,
    weapon_skill: i32,
    leadership: i32,
    objective_control: i32
}

// impl Unit {
//     pub fn get_hash_map(&self) -> HashMap<String, AttributeValue> {
//         let mut map = HashMap::new();
//         map.insert("Name".to_string(), AttributeValue::S(self.name.clone()));
//         map.insert("Movement".to_string(), AttributeValue::N(self.movement.to_string()));
//         map.insert("Toughness".to_string(), AttributeValue::N(self.toughness.to_string()));
//         map.insert("Save".to_string(), AttributeValue::N(self.save.to_string()));
//         map.insert("WeaponSkill".to_string(), AttributeValue::N(self.weapon_skill.to_string()));
//         map.insert("Leadership".to_string(), AttributeValue::N(self.leadership.to_string()));
//         map.insert("ObjectiveControl".to_string(), AttributeValue::N(self.objective_control.to_string()));
//         map
//     }
// }
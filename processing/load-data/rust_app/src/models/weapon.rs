use crate::models::effect::Effect;
use aws_sdk_dynamodb::types::AttributeValue;
use serde::Deserialize;
use std::collections::HashMap;

// Create a struct named Weapon that has the following fields
// Name: String
// Range: i32
// Attacks: i32
// Skill: i32
// Strength: i32
// ArmorPenetration: i32
// Damage: i32
// Effects: Vec<Effect>
// and that's it!
#[derive(Deserialize, Debug)]
pub struct Weapon {
    name: String,
    range: i32,
    attacks: i32,
    skill: i32,
    strength: i32,
    armor_penetration: i32,
    damage: i32,
    effects: Vec<Effect>,
}

// Create an implementation for Weapon that has a function called get_hash_map that returns a HashMap<String, AttributeValue>
// This function should return a HashMap with all properties of the Weapon struct
// The Effects property should be a list of HashMaps with the properties of the Effect struct
impl Weapon {
    pub fn get_hash_map(&self) -> HashMap<String, AttributeValue> {
        let mut map = HashMap::new();
        map.insert("Name".to_string(), AttributeValue::S(self.name.clone()));
        map.insert(
            "Range".to_string(),
            AttributeValue::N(self.range.to_string()),
        );
        map.insert(
            "Attacks".to_string(),
            AttributeValue::N(self.attacks.to_string()),
        );
        map.insert(
            "Skill".to_string(),
            AttributeValue::N(self.skill.to_string()),
        );
        map.insert(
            "Strength".to_string(),
            AttributeValue::N(self.strength.to_string()),
        );
        map.insert(
            "ArmorPenetration".to_string(),
            AttributeValue::N(self.armor_penetration.to_string()),
        );
        map.insert(
            "Damage".to_string(),
            AttributeValue::N(self.damage.to_string()),
        );
        map.insert(
            "Effects".to_string(),
            AttributeValue::L(
                self.effects
                    .iter()
                    .map(|effect| AttributeValue::M(effect.get_hash_map()))
                    .collect(),
            ),
        );
        map
    }
}

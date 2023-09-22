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
    attacks: String,
    skill: i32,
    strength: i32,
    armor_penetration: Option<i32>,
    damage: String,
    effects: Option<Vec<Effect>>,
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
            AttributeValue::S(self.attacks.clone()),
        );
        map.insert(
            "Skill".to_string(),
            AttributeValue::N(self.skill.to_string()),
        );
        map.insert(
            "Strength".to_string(),
            AttributeValue::N(self.strength.to_string()),
        );
        match self.armor_penetration {
            Some(armor_penetration) => {
                map.insert(
                    "ArmorPenetration".to_string(),
                    AttributeValue::N(armor_penetration.to_string()),
                );
            }
            None => {}
        }
        map.insert("Damage".to_string(), AttributeValue::S(self.damage.clone()));
        match &self.effects {
            Some(effects) => {
                map.insert(
                    "Effects".to_string(),
                    AttributeValue::L(
                        effects
                            .iter()
                            .map(|effect| AttributeValue::M(effect.get_hash_map()))
                            .collect(),
                    ),
                );
            }
            None => {}
        }
        map
    }
}

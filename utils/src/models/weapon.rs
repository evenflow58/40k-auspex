use crate::models::effect::Effect;
use serde::{Deserialize, Serialize};

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
#[derive(Deserialize, Serialize, Debug, Clone)]
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
    pub fn new(
        name: String,
        range: i32,
        attacks: String,
        skill: i32,
        strength: i32,
        armor_penetration: Option<i32>,
        damage: String,
        effects: Option<Vec<Effect>>,
    ) -> Self {
        Weapon {
            name,
            range,
            attacks,
            skill,
            strength,
            armor_penetration,
            damage,
            effects,
        }
    }
}
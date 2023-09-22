use crate::models::{ability::Ability, composition::Composition, weapon::Weapon};
use aws_sdk_dynamodb::types::AttributeValue;
use serde::Deserialize;
use std::collections::HashMap;

// Create a struct named Unit that has the following fields
// Name: String
// Movement: i32
// Toughness: i32
// Save: i32
// Wounds: i32
// Leadership: i32
// Objective Control: i32
// Invulerable Save: i32
// Ranged Weapons: Vec<Weapon>
// Melee Weapons: Vec<Weapon>
// Abilities: Ability
// and that's it
#[derive(Deserialize, Debug)]
pub struct Unit {
    name: String,
    movement: i32,
    toughness: i32,
    save: i32,
    wounds: i32,
    leadership: i32,
    objective_control: i32,
    invulnerable_save: Option<i32>,
    ranged_weapons: Vec<Weapon>,
    melee_weapons: Vec<Weapon>,
    abilities: Vec<Ability>,
    composition: Composition,
}

impl Unit {
    pub fn get_hash_map(&self) -> HashMap<String, AttributeValue> {
        let mut map = HashMap::new();
        map.insert("Name".to_string(), AttributeValue::S(self.name.clone()));
        map.insert(
            "Movement".to_string(),
            AttributeValue::N(self.movement.to_string()),
        );
        map.insert(
            "Toughness".to_string(),
            AttributeValue::N(self.toughness.to_string()),
        );
        map.insert("Save".to_string(), AttributeValue::N(self.save.to_string()));
        map.insert(
            "Wounds".to_string(),
            AttributeValue::N(self.wounds.to_string()),
        );
        map.insert(
            "Leadership".to_string(),
            AttributeValue::N(self.leadership.to_string()),
        );
        match self.invulnerable_save {
            Some(invulnerable_save) => {
                map.insert(
                    "InvulnerableSave".to_string(),
                    AttributeValue::N(invulnerable_save.to_string().clone()),
                );
            }
            None => {}
        };
        map.insert(
            "ObjectiveControl".to_string(),
            AttributeValue::N(self.objective_control.to_string()),
        );
        map.insert(
            "RangedWeapons".to_string(),
            AttributeValue::L(
                self.ranged_weapons
                    .iter()
                    .map(|weapon| AttributeValue::M(weapon.get_hash_map()))
                    .collect(),
            ),
        );
        map.insert(
            "MeleeWeapons".to_string(),
            AttributeValue::L(
                self.melee_weapons
                    .iter()
                    .map(|weapon| AttributeValue::M(weapon.get_hash_map()))
                    .collect(),
            ),
        );
        map.insert(
            "Abilities".to_string(),
            AttributeValue::L(
                self.abilities
                    .iter()
                    .map(|ability| AttributeValue::M(ability.get_hash_map()))
                    .collect(),
            ),
        );
        map.insert(
            "Composition".to_string(),
            AttributeValue::M(self.composition.get_hash_map()),
        );
        map
    }
}

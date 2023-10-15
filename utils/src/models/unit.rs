use crate::models::{ability::Ability, composition::Composition, weapon::Weapon};
use serde::{Deserialize, Serialize};

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
#[derive(Deserialize, Serialize, Debug, Clone)]
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
    pub fn new(name: String,
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
    ) -> Self {
        Unit {
            name,
            movement,
            toughness,
            save,
            wounds,
            leadership,
            objective_control,
            invulnerable_save,
            ranged_weapons,
            melee_weapons,
            abilities,
            composition,
        }
    }
}
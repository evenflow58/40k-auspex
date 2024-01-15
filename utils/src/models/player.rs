use serde::{Deserialize, Serialize};
use crate::enums::mission_type::MissionType;

use super::army::Army;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Player {
    pub id: Option<String>,
    pub name: Option<String>,
    pub mission_type: Option<MissionType>,
    pub turn_order: u8,
    pub current_missions: Option<Vec<String>>,
    pub discarded_missions: Option<Vec<String>>,
    pub army: Option<Army>,
    pub army_list: Option<String>,
}

impl Player {
    pub fn new(
        turn_order: u8,
        id: Option<String>,
        name: Option<String>,
        mission_type: Option<MissionType>,
        current_missions: Option<Vec<String>>,
        discarded_missions: Option<Vec<String>>,
        army: Option<Army>,
        army_list: Option<String>,
    ) -> Self {
        Player {
            id,
            name,
            mission_type,
            turn_order,
            current_missions,
            discarded_missions,
            army,
            army_list,
        }
    }
}
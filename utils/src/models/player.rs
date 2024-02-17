use serde::{Deserialize, Serialize};
use crate::enums::mission_type::MissionType;

use super::army_list::ArmyList;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Player {
    pub id: Option<String>,
    pub name: Option<String>,
    pub mission_type: Option<MissionType>,
    pub turn_order: u8,
    pub current_missions: Option<Vec<String>>,
    pub discarded_missions: Option<Vec<String>>,
    pub army: Option<ArmyList>,
}

impl Player {
    pub fn new(
        id: Option<String>,
        name: Option<String>,
        mission_type: Option<MissionType>,
        turn_order: u8,
        current_missions: Option<Vec<String>>,
        discarded_missions: Option<Vec<String>>,
        army: Option<ArmyList>,
    ) -> Self {
        Player {
            id,
            name,
            mission_type,
            turn_order,
            current_missions,
            discarded_missions,
            army,
        }
    }
}
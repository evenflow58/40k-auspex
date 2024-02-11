use serde::{Deserialize, Serialize};
use utils::enums::mission_type::MissionType;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Player {
    pub id: Option<String>,
    pub name: Option<String>,
    pub mission_type: Option<MissionType>,
    pub turn_order: String,
    pub current_missions: Option<Vec<String>>,
    pub discarded_missions: Option<Vec<String>>,
    pub army_list: Option<String>,
}

impl Player {
    pub fn new(
        id: Option<String>,
        name: Option<String>,
        mission_type: Option<MissionType>,
        turn_order: String,
        current_missions: Option<Vec<String>>,
        discarded_missions: Option<Vec<String>>,
        army_list: Option<String>,
    ) -> Self {
        Player {
            id,
            name,
            mission_type,
            turn_order,
            current_missions,
            discarded_missions,
            army_list,
        }
    }
}

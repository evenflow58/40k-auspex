use serde::{Deserialize, Serialize};
use crate::enums::game_size::GameSize;
use crate::models::player::Player;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Game {
    pub id: i32,
    pub name: String,
    pub size: GameSize,
    pub attacker: Player,
    pub defender: Player,
}

impl Game {
    pub fn new(id: i32, name: String, size: GameSize, attacker: Player, defender: Player) -> Self {
        Game {
            id,
            name,
            size,
            attacker,
            defender,
        }
    }
}
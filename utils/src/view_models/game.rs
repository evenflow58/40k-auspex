use crate::{view_models::player::Player,enums::game_size::GameSize};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Game {
    pub name: String,
    pub size: GameSize,
    pub attacker: Player,
    pub defender: Player,
}

impl Game {
    pub fn new(name: String, size: GameSize, attacker: Player, defender: Player) -> Self {
        Game {
            name,
            size,
            attacker,
            defender,
        }
    }
}

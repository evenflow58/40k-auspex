use serde::{Deserialize, Serialize};
use crate::enums::game_size::GameSize;
use crate::models::player::Player;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Game {
    pub size: GameSize,
    pub attacker: Player,
    pub defender: Player,

}

impl Game {
    pub fn new(size: GameSize, attacker: Player, defender: Player) -> Self {
        Game {
            size,
            attacker,
            defender,
        }
    }
}
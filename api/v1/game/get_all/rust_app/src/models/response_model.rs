use serde::{Deserialize, Serialize};

use utils::models::game::Game;

#[derive(Serialize, Deserialize)]
pub struct ResponseModel {
    pub id: i32,
    pub name: String,
}

impl ResponseModel {
    pub fn new(game: Game) -> Self {
        ResponseModel {
            id: game.id,
            name: game.name,
        }
    }
}

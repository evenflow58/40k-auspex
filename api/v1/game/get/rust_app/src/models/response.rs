use serde::{Deserialize, Serialize};
use utils::models::game::Game;

#[derive(Serialize, Deserialize)]
pub struct ResponseModel {
    pub id: String,
    pub game: Game,
}

impl ResponseModel {
    pub fn new(id: String, game: Game) -> Self {
        ResponseModel { id, game }
    }
}

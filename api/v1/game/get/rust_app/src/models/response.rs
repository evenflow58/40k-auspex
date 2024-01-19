use serde::{Deserialize, Serialize};
use utils::models::game::Game;

#[derive(Serialize, Deserialize)]
pub struct ResponseModel {
    pub id: i32,
    pub name: String,
    pub game: Game,
}

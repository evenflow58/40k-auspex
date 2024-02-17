use serde::{Deserialize, Serialize};
use utils::models::game::Game;

#[derive(Serialize, Deserialize)]
pub struct RequestModel {
    pub name: String,
    pub game: Game,
}

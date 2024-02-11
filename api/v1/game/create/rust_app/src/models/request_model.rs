use crate::models::game::Game;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct RequestModel {
    pub game: Game,
}

use crate::view_models::game::Game;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct RequestModel {
    pub game: Game,
}

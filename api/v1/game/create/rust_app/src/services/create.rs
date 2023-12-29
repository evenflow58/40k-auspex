use chrono::offset::Utc;
use std::error::Error;

use super::data::save_game;
use utils::models::game::Game;

pub async fn create(
    player_ids: Vec<String>,
    name: String,
    game: Game,
) -> Result<String, Box<dyn Error>> {
    match save_game(player_ids, name.to_string(), Utc::now(), game).await {
        Ok(id) => Ok(id),
        Err(err) => panic!("{}", err),
    }
}

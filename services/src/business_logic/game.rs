use chrono::offset::Utc;
use std::error::Error;

use crate::data::game;
use utils::models::game::Game;

pub async fn create(
    player_ids: Vec<String>,
    name: String,
    game: Game,
) -> Result<String, Box<dyn Error>> {
    match game::upsert(player_ids, name.to_string(), Utc::now(), game).await {
        Ok(id) => Ok(id),
        Err(err) => panic!("{}", err),
    }
}

pub async fn get(id: String, user_id: String) -> Result<Game, Box<dyn Error>> {
    match game::get(id, user_id).await {
        Ok(game) => Ok(game),
        Err(err) => panic!("{}", err),
    }
}

pub async fn get_all(user_id: String) -> Result<Vec<Game>, Box<dyn Error>> {
    match game::get_all(user_id).await {
        Ok(games) => Ok(games),
        Err(err) => panic!("{}", err),
    }
}
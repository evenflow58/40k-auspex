use chrono::offset::Utc;
use std::error::Error;

use super::data::get_all;
use utils::models::game::Game;

pub async fn get_all(id: String, name: String, game: Game) -> Result<String, Box<dyn Error>> {
    match get_all(id).await {
        Ok(id) => Ok(id),
        Err(err) => panic!("{}", err),
    }
}

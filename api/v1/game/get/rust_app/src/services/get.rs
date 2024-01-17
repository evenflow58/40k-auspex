use std::error::Error;

use super::data;
use utils::models::game::Game;

pub async fn get(id: String, user_id: String) -> Result<Game, Box<dyn Error>> {
    match data::get(id, user_id).await {
        Ok(game) => Ok(game),
        Err(err) => panic!("{}", err),
    }
}

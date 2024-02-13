use chrono::offset::Utc;
use std::error::Error;

use crate::data::game;
use utils::models::{game::Game,object_with_id::ObjectWithId};

pub async fn create(
    player_ids: Vec<String>,
    game: Game,
) -> Result<String, Box<dyn Error>> {
    match game::create(player_ids, Utc::now(), game).await {
        Ok(id) => Ok(id),
        Err(err) => panic!("{:?}", err),
    }
}

pub async fn update(
    id: String,
    player_ids: Vec<String>,
    mut game: Game,
) -> Result<String, Box<dyn Error>> {
    if game.attacker.army.is_none() || game.defender.army.is_none() {
        match get(id.clone(), player_ids.first().unwrap().to_string()).await {
            Ok(ObjectWithId { object: saved_game, .. }) => {
                if game.attacker.army.is_none() && !saved_game.attacker.army.is_none() {
                    game.attacker.army = saved_game.attacker.army.clone();
                }

                if game.defender.army.is_none() && !saved_game.defender.army.is_none() {
                    game.defender.army = saved_game.defender.army.clone();
                }
            },
            Err(err) => panic!("{:?}", err),    
        }
    }

    match game::update(id, player_ids, Utc::now(), game).await {
        Ok(id) => Ok(id),
        Err(err) => panic!("{:?}", err),
    }
}

pub async fn get(id: String, user_id: String) -> Result<ObjectWithId<Game>, Box<dyn Error>> {
    match game::get(id, user_id).await {
        Ok(game) => Ok(game),
        Err(err) => panic!("{:?}", err),
    }
}

pub async fn get_all(user_id: String) -> Result<Vec<ObjectWithId<Game>>, Box<dyn Error>> {
    match game::get_all(user_id).await {
        Ok(games) => Ok(games),
        Err(err) => panic!("{:?}", err),
    }
}
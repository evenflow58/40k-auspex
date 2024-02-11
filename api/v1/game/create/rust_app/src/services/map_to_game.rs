use crate::models::request_model::RequestModel;
use crate::services::serialize::serialize_army;
use utils::models::{army_list::ArmyList, game::Game, player::Player};

async fn get_army_or_none(army_list: Option<String>) -> Option<ArmyList> {
    match army_list {
        Some(list) => match serialize_army(list).await {
            Ok(Some(army)) => Some(army),
            Ok(None) => None,
            Err(_) => panic!("Unable to serialize army."),
        },
        None => None,
    }
}

pub async fn map_to_game(request_model: RequestModel) -> Game {
    let game = request_model.game.clone();

    Game::new(
        game.name,
        game.size,
        Player::new(
            game.attacker.id,
            game.attacker.name,
            game.attacker.mission_type,
            game.attacker.turn_order.parse::<u8>().unwrap(),
            game.attacker.current_missions,
            game.attacker.discarded_missions,
            get_army_or_none(game.attacker.army_list.clone()).await,
        ),
        Player::new(
            game.defender.id,
            game.defender.name,
            game.defender.mission_type,
            game.defender.turn_order.parse::<u8>().unwrap(),
            game.defender.current_missions,
            game.defender.discarded_missions,
            get_army_or_none(game.defender.army_list.clone()).await,
        ),
    )
}

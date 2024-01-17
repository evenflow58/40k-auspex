use aws_sdk_dynamodb::{types::AttributeValue, Client as dynamodb_sdk_client};
use std::error::Error;

use utils::models::game::Game;

pub async fn get(game_id: String, user_id: String) -> Result<Game, Box<dyn Error>> {
    let config = ::aws_config::load_from_env().await;
    let dynamodb_client = dynamodb_sdk_client::new(&config);
    let table_name = envmnt::get_or_panic("TABLE_NAME").to_string();

    match dynamodb_client
        .query()
        .table_name(&table_name)
        .key_condition_expression(
            "#id = :id AND #entry_type = :entry_type AND contains(#player_ids, :user_id)",
        )
        .expression_attribute_names("#id", "id")
        .expression_attribute_names("#entry_type", "entry_type")
        .expression_attribute_names("#player_ids", "player_ids")
        .expression_attribute_values(":id", AttributeValue::S(game_id))
        .expression_attribute_values(":entry_type", AttributeValue::S("Game".to_string()))
        .expression_attribute_values(":user_id", AttributeValue::S(user_id))
        .send()
        .await
    {
        Ok(result) => Ok(result
            .items
            .unwrap()
            .iter()
            .map(|item| {
                let game: Game = serde_dynamo::from_item(item.clone()).unwrap();
                game
            })
            .collect::<Vec<Game>>()
            .first()
            .unwrap()
            .clone()),
        Err(err) => panic!("Unable to get item: {:?}", err.raw_response()),
    }
}

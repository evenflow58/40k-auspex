use aws_sdk_dynamodb::{types::AttributeValue, Client as dynamodb_sdk_client};
use chrono::{DateTime, Utc};
use serde_dynamo::to_item;
use std::error::Error;
use uuid::Uuid;

use utils::models::game::Game;

pub async fn save_game(
    player_ids: Vec<String>,
    name: String,
    date: DateTime<Utc>,
    game: Game,
) -> Result<String, Box<dyn Error>> {
    let config = ::aws_config::load_from_env().await;
    let dynamodb_client = dynamodb_sdk_client::new(&config);
    let table_name = envmnt::get_or_panic("TABLE_NAME").to_string();

    match dynamodb_client
        .update_item()
        .table_name(&table_name)
        .key("id", AttributeValue::S(Uuid::new_v4().to_string()))
        .key("entry_type", AttributeValue::S("Game".to_string()))
        .update_expression(
            "SET \
            start_date = :start_date, \
            game_name = :game_name, \
            entry_data = :entry_data, \
            player_ids = :player_ids",
        )
        .expression_attribute_values(":start_date", AttributeValue::S(date.to_string()))
        .expression_attribute_values(":game_name", AttributeValue::S(name.to_string()))
        .expression_attribute_values(":entry_data", AttributeValue::M(to_item(&game)?))
        .expression_attribute_values(
            ":player_ids",
            AttributeValue::L(
                player_ids
                    .into_iter()
                    .map(|player_id| AttributeValue::S(player_id))
                    .collect(),
            ),
        )
        .return_values("ALL_NEW".into())
        .send()
        .await
    {
        Ok(returned_values) => Ok(returned_values
            .attributes
            .unwrap()
            .get("id")
            .unwrap()
            .as_s()
            .unwrap()
            .to_string()),
        Err(err) => panic!("Unable to create item: {:?}", err.raw_response()),
    }
}

use aws_sdk_dynamodb::{types::{AttributeValue, Select}, Client as dynamodb_sdk_client};
use chrono::{DateTime, Utc};
use serde_dynamo::to_item;
use serde::Deserialize;
use std::error::Error;
use uuid::Uuid;
use tracing::info;

use utils::models::game::Game;

pub async fn upsert(
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

pub async fn get(game_id: String, user_id: String) -> Result<Game, Box<dyn Error>> {
    #[derive(Deserialize)]
    struct Item {
        entry_data: Game,
    }

    let config = ::aws_config::load_from_env().await;
    let dynamodb_client = dynamodb_sdk_client::new(&config);
    let table_name = envmnt::get_or_panic("TABLE_NAME").to_string();

    match dynamodb_client
        .query()
        .table_name(&table_name)
        .key_condition_expression("#id = :id AND #entry_type = :entry_type")
        .filter_expression("contains(#player_ids, :user_id)")
        .expression_attribute_names("#id", "id")
        .expression_attribute_names("#entry_type", "entry_type")
        .expression_attribute_names("#player_ids", "player_ids")
        .expression_attribute_values(":id", AttributeValue::S(game_id))
        .expression_attribute_values(":entry_type", AttributeValue::S("Game".to_string()))
        .expression_attribute_values(":user_id", AttributeValue::S(user_id))
        .projection_expression("entry_data")
        .select(Select::SpecificAttributes)
        .send()
        .await
    {
        Ok(result) => Ok(result
            .items
            .unwrap()
            .iter()
            .map(|item| {
                info!("Item: {:?}", item.clone());
                let game_item: Item = serde_dynamo::from_item(item.clone()).unwrap();
                game_item.entry_data
            })
            .collect::<Vec<Game>>()
            .first()
            .unwrap()
            .clone()),
        Err(err) => panic!("Unable to get item: {:?}", err.raw_response()),
    }
}

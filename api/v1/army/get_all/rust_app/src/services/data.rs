use aws_sdk_dynamodb::{Client as dynamodb_sdk_client, types::Select};
use aws_sdk_dynamodb::types::AttributeValue;
use tracing::info;
use std::error::Error;

use crate::models::army_entry::ArmyEntry;

pub async fn get_armies(
    // take: i64,
    // skip: i64,
) -> Result<Vec<ArmyEntry>, Box<dyn Error>> {
    let config = ::aws_config::load_from_env().await;
    let dynamodb_client = dynamodb_sdk_client::new(&config);
    let table_name = envmnt::get_or_panic("TABLE_NAME").to_string();

    match dynamodb_client
        .query()
        .table_name(&table_name)
        .index_name("type-index")
        .expression_attribute_names("#T", "type")
        .expression_attribute_values(":V", AttributeValue::S("Army".to_string()))
        .key_condition_expression("#T = :V")
        .select(Select::AllAttributes)
        .send()
        .await
    {
        Ok(output) => {
            info!("Query succeeded. Items returned from DynamoDB: {:?}", output.items);

            let mapped_items = output.items.unwrap().iter().map(|item| {
                let name = item.get("id").unwrap().as_s().unwrap().to_string();
                info!("name: {:?}", name);

                let factions = item.get("factions").unwrap();
                info!("factions: {:?}", factions);

                // info!("Factions: {:?}", item.get("factions"));
                ArmyEntry {
                    name: "".to_string(),
                    factions: vec!["".to_string()],
                    // name: item.get("id").unwrap().as_s().unwrap().to_string(),
                    // factions: item.get("factions").unwrap().as_l(),
                }
            }).collect();

            Ok(mapped_items)
        }
        Err(error) => panic!("Error querying DynamoDB: {:?}", error),    
    }
}
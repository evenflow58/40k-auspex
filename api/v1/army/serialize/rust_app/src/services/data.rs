use aws_sdk_dynamodb::{
    types::{AttributeValue, Select},
    Client as dynamodb_sdk_client,
};
use std::error::Error;
use tracing::info;
use serde_dynamo::from_items;

use utils::models::{army::Army, dynamo_result::DynamoResult};

pub async fn get_armies() -> Result<Vec<Army>, Box<dyn Error>> {
    info!("Getting armies");
    let config = ::aws_config::load_from_env().await;
    let dynamodb_client = dynamodb_sdk_client::new(&config);
    let table_name = envmnt::get_or_panic("TABLE_NAME").to_string();
    info!("Base set");

    let result = dynamodb_client
        .query()
        .table_name(&table_name)
        .index_name("type-index")
        .expression_attribute_names("#T", "type")
        .expression_attribute_values(":V", AttributeValue::S("Army".to_string()))
        .key_condition_expression("#T = :V")
        .select(Select::AllAttributes)
        .send()
        .await?;

    if let Some(items) = result.items {
        info!("items {:?}", items);
        let armies_result: Vec<DynamoResult<Army>> = from_items(items)?;
        info!("armies_result {:?}", armies_result);

        Ok(armies_result.iter().map(|army| army.data.clone()).collect())
    } else {
        info!("else");
        Ok(vec![])
    }
}

// pub async func save_army(user_id: String) -> Result<(), Box<dyn Error>> {
//     let config = ::aws_config::load_from_env().await;
//     let dynamodb_client = dynamodb_sdk_client::new(&config);
//     let table_name = envmnt::get_or_panic("TABLE_NAME").to_string();

//     let request = dynamodb_client
//         .put_item()
//         .table_name(&table_name)
//         .item("id", AttributeValue::S(user_id))
//         .item("type", AttributeValue::S("List"))
//         .item("data", )
// }

use aws_sdk_dynamodb::{
    types::{AttributeValue, Select},
    Client as dynamodb_sdk_client,
};
use serde_dynamo::{from_items, to_item};
use std::error::Error;
use tracing::info;

use utils::models::{army::Army, army_list::ArmyList, dynamo_result::DynamoResult};

pub async fn get_armies() -> Result<Vec<Army>, Box<dyn Error>> {
    let config = ::aws_config::load_from_env().await;
    let dynamodb_client = dynamodb_sdk_client::new(&config);
    let table_name = envmnt::get_or_panic("TABLE_NAME").to_string();

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
        match from_items(items) {
            Ok(armies_result) => Ok(armies_result
                .iter()
                .map(|army: &DynamoResult<Army>| army.data.clone())
                .collect()),
            Err(error) => panic!("Unable to serialize army. {:?}", error),
        }
    } else {
        Ok(vec![])
    }
}

pub async fn save_army_list(user_id: String, data: ArmyList) -> Result<(), Box<dyn Error>> {
    let config = ::aws_config::load_from_env().await;
    let dynamodb_client = dynamodb_sdk_client::new(&config);
    let table_name = envmnt::get_or_panic("TABLE_NAME").to_string();

    match dynamodb_client
        .put_item()
        .table_name(&table_name)
        .item("id", AttributeValue::S(user_id))
        .item("type", AttributeValue::S("List".to_string()))
        .item("data", AttributeValue::M(to_item(&data)?))
        .send()
        .await
    {
        Ok(_) => Ok(()),
        Err(err) => panic!("Unable to put item: {:?}", err.raw_response()),
    }
}

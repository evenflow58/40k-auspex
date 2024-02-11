use aws_sdk_dynamodb::{
    types::{AttributeValue, Select},
    Client as dynamodb_sdk_client,
};
use serde_dynamo::{from_items, to_item};
use std::error::Error;
use uuid::Uuid;
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
        .expression_attribute_names("#T", "entry_type")
        .expression_attribute_values(":V", AttributeValue::S("Army".to_string()))
        .key_condition_expression("#T = :V")
        .select(Select::AllAttributes)
        .send()
        .await?;

    if let Some(items) = result.items {
        match from_items(items) {
            Ok(armies_result) => Ok(armies_result
                .iter()
                .map(|army: &DynamoResult<Army>| army.entry_data.clone())
                .collect()),
            Err(error) => panic!("Unable to serialize army. {:?}", error),
        }
    } else {
        Ok(vec![])
    }
}

pub async fn save_army_list(
    id: Option<&String>,
    user_id: String,
    name: String,
    data: ArmyList,
) -> Result<String, Box<dyn Error>> {
    let config = ::aws_config::load_from_env().await;
    let dynamodb_client = dynamodb_sdk_client::new(&config);
    let table_name = envmnt::get_or_panic("TABLE_NAME").to_string();

    match dynamodb_client
        .update_item()
        .table_name(&table_name)
        .key(
            "id",
            AttributeValue::S(id.unwrap_or(&Uuid::new_v4().to_string()).to_string()),
        )
        .key("entry_type", AttributeValue::S("List".to_string()))
        .update_expression(
            "SET \
            user_email = if_not_exists(user_email, :user_email), \
            list_name = :list_name, \
            entry_data = :entry_data",
        )
        .expression_attribute_values(":user_email", AttributeValue::S(user_id))
        .expression_attribute_values(":list_name", AttributeValue::S(name))
        .expression_attribute_values(":entry_data", AttributeValue::M(to_item(&data)?))
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
        Err(err) => panic!("Unable to put item: {:?}", err.raw_response()),
    }
}

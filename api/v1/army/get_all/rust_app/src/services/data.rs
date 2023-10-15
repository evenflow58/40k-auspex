use aws_sdk_dynamodb::{Client as dynamodb_sdk_client, types::Select};
use aws_sdk_dynamodb::types::AttributeValue;
use tracing::info;
use std::error::Error;
use utils::models::{army::Army, dynamo_result::DynamoResult};

pub async fn get_armies(
    // take: i64,
    // skip: i64,
) -> Result<Vec<Army>, Box<dyn Error>> {
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
        info!("items: {:?}", items);
        let armies_result: Vec<DynamoResult<Army>> = serde_dynamo::from_items(items)?;
        info!("armies: {:?}", armies_result);

        Ok(armies_result.iter().map(|army| army.data.clone()).collect())
    } else {
        Ok(vec![])
    }   
}
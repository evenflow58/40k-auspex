use aws_sdk_dynamodb::{types::AttributeValue, Client as dynamodb_sdk_client};
use aws_sdk_s3::Client as s3_sdk_client;
use serde_dynamo::to_item;
use serde_json::from_str;
use std::error::Error;
use tracing::info;
use utils::models::army::Army;

pub async fn serialize_and_load_data(
    dynamodb_client: &dynamodb_sdk_client,
    s3_client: &s3_sdk_client,
    table_name: &str,
    bucket: String,
    file: String,
) -> Result<(), Box<dyn Error>> {
    info!("serialize_and_load_data");

    // Get the object from the S3 bucket
    let army_file = match s3_client.get_object().bucket(bucket).key(file).send().await {
        Ok(output) => output,
        Err(error) => panic!("Error getting object from S3: {:?}", error),
    };

    let bytes = army_file.body.collect().await?.into_bytes();
    let response = std::str::from_utf8(&bytes)?;

    let army: Army = from_str(&response).unwrap();

    match dynamodb_client
        .put_item()
        .table_name(&*table_name)
        .item("id", AttributeValue::S(army.name.clone()))
        .item("entry_type", AttributeValue::S("Army".to_string()))
        .item("entry_data", AttributeValue::M(to_item(&army)?))
        .send()
        .await
    {
        Ok(_) => Ok(()),
        Err(error) => panic!("Error writing to DynamoDB: {:?}", error),
    }
}

use aws_sdk_dynamodb::{
    types::{AttributeValue, PutRequest, WriteRequest},
    Client as dynamodb_sdk_client,
};
use aws_sdk_s3::{Client as s3_sdk_client};
use tracing::info;
use std::collections::HashMap;
use std::error::Error;
use crate::models::army::Army;

pub async fn serialize_and_load_data (
    dynamodb_client: &dynamodb_sdk_client,
    s3_client: &s3_sdk_client,
    table_name: &str,
    bucket: String,
    file: String,
) -> Result<(), Box<dyn Error>> {
    // Get the object from the S3 bucket
    let army_file = s3_client
        .get_object()
        .bucket(bucket)
        .key(file)
        .send()
        .await?;
    let bytes = army_file.body.collect().await?.into_bytes();
    let response = std::str::from_utf8(&bytes)?;

    // let file = File::open(file_location);
    let army: Army = serde_json::from_str(&response).unwrap();

    info!("{:?} serialized", army);

    // Using the client variable call the batch_write_item() function with the
    // table_name variable and a HashMap with the id of "grey_knights" and the
    // type of "army" and the data of the grey_knights variable.
    dynamodb_client
        .batch_write_item()
        .request_items(
            table_name,
            vec![WriteRequest::builder()
                .put_request(
                    PutRequest::builder()
                        .set_item(Some(HashMap::from([
                            (
                                "id".to_string(),
                                AttributeValue::S(army.name.clone()),
                            ),
                            ("type".to_string(), AttributeValue::S("Army".to_string())),
                            (
                                "data".to_string(),
                                AttributeValue::M(army.get_hash_map()),
                            ),
                        ])))
                        .build(),
                )
                .build()],
        )
        .send()
        .await?;

    info!("Finished writing {:?}", army.name);

    Ok(())
}

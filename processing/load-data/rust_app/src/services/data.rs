use aws_sdk_dynamodb::{
    types::{AttributeValue, PutRequest, WriteRequest},
    Client as dynamodb_sdk_client,
};
use tracing::info;
use std::fs::File;
use std::io::Read;

pub async fn serialize_and_load_data(dynamodb_client: dynamodb_sdk_client, table_name: &str, army_name: &str, file_location: &str) {
    // Create a variable called file that opens the file data/grey_knights.json
    // let army: Army = serde_json::from_str(&String::from_utf8_lossy(include_bytes!(
    //     file_location
    // )))?;

    // let file = File::open(file_location);
    // let army: Army = serde_json::from_reader(file);

    // info!("{} serialized", army_name);

    // // Using the client variable call the batch_write_item() function with the
    // // table_name variable and a HashMap with the id of "grey_knights" and the
    // // type of "army" and the data of the grey_knights variable.
    // let result = dynamodb_client
    //     .batch_write_item()
    //     .request_items(
    //         table_name,
    //         vec![WriteRequest::builder()
    //             .put_request(
    //                 PutRequest::builder()
    //                     .set_item(Some(HashMap::from([
    //                         (
    //                             "id".to_string(),
    //                             AttributeValue::S(army_name),
    //                         ),
    //                         ("type".to_string(), AttributeValue::S("Army".to_string())),
    //                         (
    //                             "data".to_string(),
    //                             AttributeValue::M(grey_knights.get_hash_map()),
    //                         ),
    //                     ])))
    //                     .build(),
    //             )
    //             .build()],
    //     )
    //     .send()
    //     .await?;

    info!("Finished writing {:?}", army_name);
}
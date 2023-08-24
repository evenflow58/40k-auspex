use std::collections::HashMap;
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use aws_sdk_dynamodb::{Client};
use aws_sdk_dynamodb::{types::{WriteRequest, PutRequest, AttributeValue}};
use envmnt;
use std::fs::File;
use std::io::BufReader;

mod models;
use crate::models::army::Army;
use crate::models::request::Request;
use crate::models::response::Response;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}

// Create a function call function_handler that takes a LambdaEvent and returns a Result
// with a Response or an Error
async fn function_handler(_event: LambdaEvent<Request>) -> Result<Response, Error> {
    // Create a variable called file that opens the file data/grey_knights.json
    let file = File::open("./data/grey_knights.json").unwrap();
    // Create a variable called reader that creates a BufReader from the file variable
    let reader = BufReader::new(file);
    // Create a variable called grey_knights that is a Army struct that is created from
    // the reader variable
    let grey_knights: Army = serde_json::from_reader(reader).unwrap();

    // Create a variable called config that is a aws_config::Config that is created from
    // the load_from_env() function
    let config = ::aws_config::load_from_env().await;
    // Create a variable called client that is a dynamodb::Client that is created from
    // the config variable
    let client = Client::new(&config);

    // Create a variable called table_name that is a String that is created from the
    // TABLE_NAME environment variable
    let table_name = envmnt::get_or_panic("TABLE_NAME").to_string();

    // Using the client variable call the batch_write_item() function with the
    // table_name variable and a HashMap with the id of "grey_knights" and the
    // type of "army" and the data of the grey_knights variable.
    client
        .batch_write_item()
        .request_items(
            table_name,
            vec![
                WriteRequest::builder()
                    .put_request(
                        PutRequest::builder()
                            .set_item(Some(HashMap::from(
                                [
                                    (
                                        "id".to_string(),
                                        AttributeValue::S("GreyKnights".to_string())
                                    ),
                                    (
                                        "type".to_string(),
                                        AttributeValue::S("Army".to_string())
                                    ),
                                    (
                                        "data".to_string(),
                                        AttributeValue::M(grey_knights.get_hash_map())
                                    )
                                ]
                            )))
                            .build()
                    )
                    .build()
            ],
        )
        .send()
        .await?;

    Ok(Response {})
}
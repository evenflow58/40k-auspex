use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use aws_sdk_dynamodb as dynamodb;
use serde::{Deserialize, Serialize};
use envmnt;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

/// This is a made-up example. Requests come into the runtime as unicode
/// strings in json format, which can map to any structure that implements `serde::Deserialize`
/// The runtime pays no attention to the contents of the request payload.
#[derive(Deserialize)]
struct Request {
}

/// This is a made-up example of what a response structure may look like.
/// There is no restriction on what it can be. The runtime requires responses
/// to be serialized into json. The runtime pays no attention
/// to the contents of the response payload.
#[derive(Serialize)]
struct Response {
    statusCode: i32,
    body: String,
}

// Build a struct to hold unit data
#[derive(Serialize, Deserialize, Debug)]
struct Unit {
    Name: String,
    Movement: i32,
    Toughness: i32,
    Save: i32,
    WeaponSkill: i32,
    Leadership: i32,
    ObjectiveControl: i32,
}

// Build a struct to hold the data from the data/grey_knights.json file
#[derive(Serialize, Deserialize, Debug)]
struct Army {
    Units: Vec<Unit>
}

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
/// - https://github.com/aws-samples/serverless-rust-demo/
async fn function_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    // Import a json file name grey_knights.json
    let file = File::open("data/grey_knights.json").unwrap();
    let mut reader = BufReader::new(file);
    let mut grey_knights: Army = serde_json::from_reader(reader).unwrap();

    // Build a dynamo db client
    let config = ::aws_config::load_from_env().await;
    let client = dynamodb::Client::new(&config);

    // Add the grey knights data to a dynamo table name 40kAuspex
    let table_name = envmnt::get_or_panic("TABLE_NAME").to_string();
    client
        .batch_write_item()
        .request_items(
            table_name,
            vec![WriteRequest::builder()
                .put_request(
                    PutRequest::builder()
                        .set_item(Some(HashMap::from(
                            [
                                (
                                    "id".to_string(),
                                    AttributeValue::S(
                                        "GreyKnights".to_string()),
                                ),
                                (
                                    "type".to_string(),
                                    AttributeValue::S(
                                        "Army".to_string()),
                                ),
                                "data".to_string(),
                                AttributeValue::L(
                                    &grey_knights
                                )
                            ]
                        )))
                        .build()
                ).build()
                ])
                .send()
                .await?;

    // Return `Response` (it will be serialized to JSON automatically by the runtime)
    Ok(Response {
        statusCode: 200,
        body: "Hello from Rust!".to_string(),
    });
}

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
use aws_lambda_events::event::codepipeline_job::CodePipelineJob;
use aws_sdk_codepipeline::{Client as codepipeline_sdk_client};
use aws_sdk_dynamodb::{
    types::{AttributeValue, PutRequest, WriteRequest},
    Client as dynamodb_sdk_client,
};
use envmnt;
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use std::collections::HashMap;
use tracing::info;

use load_data::models::{army::Army, response::Response};

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
async fn function_handler(event: LambdaEvent<CodePipelineJob>) -> Result<Response, Error> {
    info!("Starting {:?}", event);
    info!("job id {:?}", event.payload.id);

    // Create a variable called file that opens the file data/grey_knights.json
    let grey_knights: Army = serde_json::from_str(&String::from_utf8_lossy(include_bytes!(
        "data/grey_knights.json"
    )))?;
    info!("Grey knights serialized {:?}", grey_knights);
    // Create a variable called reader that creates a BufReader from the file variable
    // let reader = BufReader::new(file);
    // Create a variable called grey_knights that is a Army struct that is created from
    // the reader variable
    // let grey_knights: Army = serde_json::from_reader(reader).unwrap();

    // Create a variable called config that is a aws_config::Config that is created from
    // the load_from_env() function
    let config = ::aws_config::load_from_env().await;
    info!("Loaded config {:?}", config);
    // // Create a variable called client that is a dynamodb::Client that is created from
    // // the config variable
    let dynamodb_client = dynamodb_sdk_client::new(&config);
    info!("Created client");

    // Create a variable called table_name that is a String that is created from the
    // TABLE_NAME environment variable
    let table_name = envmnt::get_or_panic("TABLE_NAME").to_string();

    // Using the client variable call the batch_write_item() function with the
    // table_name variable and a HashMap with the id of "grey_knights" and the
    // type of "army" and the data of the grey_knights variable.
    let result = dynamodb_client
        .batch_write_item()
        .request_items(
            table_name,
            vec![WriteRequest::builder()
                .put_request(
                    PutRequest::builder()
                        .set_item(Some(HashMap::from([
                            (
                                "id".to_string(),
                                AttributeValue::S("GreyKnights".to_string()),
                            ),
                            ("type".to_string(), AttributeValue::S("Army".to_string())),
                            (
                                "data".to_string(),
                                AttributeValue::M(grey_knights.get_hash_map()),
                            ),
                        ])))
                        .build(),
                )
                .build()],
        )
        .send()
        .await?;

    info!("Finished writing. {:?}", result);

    let codepipeline_client = codepipeline_sdk_client::new(&config);
    codepipeline_client.put_job_success_result()
        .job_id(event.payload.id.unwrap())
        .send()
        .await?;

    Ok(Response {})
}

use aws_lambda_events::event::s3::batch_job::S3BatchJobEvent;
use aws_sdk_dynamodb::{
    types::{AttributeValue, PutRequest, WriteRequest},
    Client as dynamodb_sdk_client,
};
use envmnt;
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use std::collections::HashMap;
use tracing::info;

use load_data::{
    models::{army::Army, response::Response},
    services::data::serialize_and_load_data,
};

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
async fn function_handler(event: LambdaEvent<S3BatchJobEvent>) -> Result<Response, Error> {
    info!("Starting {:?}", event);

    // Create a variable called config that is a aws_config::Config that is created from
    // the load_from_env() function
    let config = ::aws_config::load_from_env().await;
    info!("Loaded config {:?}", config);

    // Create a variable called dynamodb-client that is a dynamodb::Client that is created from
    // the config variable
    let dynamodb_client = dynamodb_sdk_client::new(&config);
    info!("Created client");

    // Create a variable called table_name that is a String that is created from the
    // TABLE_NAME environment variable
    let table_name = envmnt::get_or_panic("TABLE_NAME").to_string();

    // serialize_and_load_data(&dynamodb_client, table_name, "Grey Knights", "data/grey_knights.json").await?;

    Ok(Response {})
}

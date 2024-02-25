use aws_lambda_events::event::s3::S3Event;
use aws_lambda_events::event::sns::SnsEvent;
use aws_sdk_dynamodb::Client as dynamodb_sdk_client;
use aws_sdk_s3::Client as s3_sdk_client;
use envmnt;
use futures::future::join_all;
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use tracing::info;

use utils::models::response::Response;

use armies::services::data::serialize_and_load_data;

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
async fn function_handler(event: LambdaEvent<SnsEvent>) -> Result<Response, Error> {
    info!("Event {:?}", event);

    // Create a variable called config that is a aws_config::Config that is created from
    // the load_from_env() function
    let config = ::aws_config::load_from_env().await;

    // Create a variable called dynamodb-client that is a dynamodb::Client that is created from
    // the config variable
    let dynamodb_client = dynamodb_sdk_client::new(&config);
    let s3_client = s3_sdk_client::new(&config);
    info!("Created clients");

    // Create a variable called table_name that is a String that is created from the
    // TABLE_NAME environment variable
    let table_name = envmnt::get_or_panic("TABLE_NAME").to_string();

    let mut futures = Vec::new();

    // Create a vector of futures by iterating through the records in the payload of the event.
    // For each record, iterate through the messages in the record. For each message, deserialize
    // the message into a S3Event. For each record in the S3Event, create a future with
    // serialize_and_load_data and append it to the vector of futures.
    for record in event.payload.records {
        let s3_event: S3Event = serde_json::from_str(&record.sns.message).unwrap();
        for record in s3_event.records {
            let bucket = record.s3.bucket.name.unwrap();
            let file = record.s3.object.key.unwrap();
            futures.push(serialize_and_load_data(
                &dynamodb_client,
                &s3_client,
                &table_name,
                bucket,
                file,
            ));
        }
    }

    info!("Built futures");

    join_all(futures).await;

    info!("Joined futures");

    Ok(Response {})
}

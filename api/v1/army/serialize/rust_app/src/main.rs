use aws_lambda_events::event::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use http::HeaderMap;
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use tracing::info;

use serialize::services::serialize::serialize_army;
use utils::traits::api_context::ApiContext;

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
#[tracing::instrument(skip(event), fields(req_id = %event.context.request_id))]
async fn function_handler(
    event: LambdaEvent<ApiGatewayProxyRequest>,
) -> Result<ApiGatewayProxyResponse, Error> {
    info!("Event: {:?}", event);

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("Access-Control-Allow-Origin", "*".parse().unwrap());

    let _ = serialize_army(&event.get_email(), &event.payload.body.unwrap()).await;

    let resp = ApiGatewayProxyResponse {
        status_code: 200,
        headers: headers.clone(),
        multi_value_headers: headers.clone(),
        body: None,
        is_base64_encoded: false,
    };

    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}

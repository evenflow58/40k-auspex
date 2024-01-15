use aws_lambda_events::encodings::Body;
use aws_lambda_events::event::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use http::HeaderMap;
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use serde_json::{from_str, json};
use tracing::info;

use getAll::services::get_all::get_all;

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

    match get_all(&event.get_email()).await {
        Ok({ id, name }) => Ok(ApiGatewayProxyResponse {
            status_code: 200,
            headers: headers.clone(),
            multi_value_headers: headers.clone(),
            body: Some(Body::Text(json!({ "id": id }).to_string())),
            is_base64_encoded: false,
        }),
        Err(err) => {
            info!("Error {:?}", err);
            Ok(ApiGatewayProxyResponse {
                status_code: 500,
                headers: headers.clone(),
                multi_value_headers: headers.clone(),
                body: None,
                is_base64_encoded: false,
            })
        }
    }
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

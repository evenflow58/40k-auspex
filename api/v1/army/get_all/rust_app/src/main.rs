use lambda_runtime::{run, service_fn, LambdaEvent, Error};
use aws_lambda_events::event::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use http::HeaderMap;
use tracing::info;

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
async fn function_handler(event: LambdaEvent<ApiGatewayProxyRequest>) -> Result<ApiGatewayProxyResponse, Error> {
    info!("Event: {:?}", event);
    info!("Params: {:?}", event.payload.query_string_parameters);
    
    

    let mut headers = HeaderMap::new();
    headers.insert("content-type", "application/json".parse().unwrap());

    let resp = ApiGatewayProxyResponse {
        status_code: 200,
        headers: headers.clone(),
        multi_value_headers: headers.clone(),
        body: Some("Some response".into()),
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

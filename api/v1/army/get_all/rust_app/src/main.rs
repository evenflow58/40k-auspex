use lambda_http::{run, service_fn, Error, Request, Response, Body, RequestExt};
use http::HeaderMap;
use tracing::info;

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    info!("Event: {:#?}", event);
    info!("Context: {:?}", event.lambda_context());
    info!("Params: {:?}", event.query_string_parameters_ref());
    // Extract some useful information from the request

    // let (parts, body) = event.into_parts();
    // let body = serde_json::from_slice(&body)?;
    // info!("Request: {:?}", Request::from_parts(parts, body));

    let mut headers = HeaderMap::new();
    headers.insert("content-type", "application/json".parse().unwrap());

    let resp = Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body("Some response".into())
        .map_err(Box::new)?;

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

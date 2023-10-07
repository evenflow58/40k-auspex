use aws_lambda_events::event::codepipeline_job::CodePipelineJobEvent;
use aws_sdk_s3::Client as s3_sdk_client;
use aws_sdk_codepipeline::Client as codepipeline_job_sdk_client;
use envmnt;
use futures::future::join_all;
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use tracing::info;
use include_dir::Dir;
use bytes::Bytes;
use aws_smithy_http::byte_stream::ByteStream;


use drop_files::models::response::Response;

#[macro_use]
extern crate include_dir;

static DATA_DIR: Dir = include_dir!("src/data");

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
async fn function_handler(event: LambdaEvent<CodePipelineJobEvent>) -> Result<Response, Error> {
    info!("Event {:?}", event);

    // Create a variable called config that is a aws_config::Config that is created from
    // the load_from_env() function
    let config = ::aws_config::load_from_env().await;
    info!("Loaded config {:?}", config);

    // Create a variable called s3_client that is a s3::Client that is created from the config
    let s3_client = s3_sdk_client::new(&config);
    let codepipeline_job_client = codepipeline_job_sdk_client::new(&config);

    let mut futures = Vec::new();

    // Loop through the DATA_DIR.files() and put them into the S3 bucket
    for file in DATA_DIR.files() {
        let bucket = envmnt::get_or_panic("BUCKET_NAME");
        let key = file.path().to_str().unwrap();
        let body = ByteStream::from(Bytes::from_static(file.contents()));
        futures.push(s3_client
            .put_object()
            .bucket(bucket)
            .key(key)
            .body(body)
            .send());
    }

    // await futures
    join_all(futures).await;

    codepipeline_job_client
        .put_job_success_result()
        .set_job_id(event.payload.code_pipeline_job.id)
        .send()
        .await?;

    Ok(Response {})
}

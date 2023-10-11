use aws_sdk_dynamodb::Client as dynamodb_sdk_client;

pub async fn get_armies(
    take: i64,
    skip: i64,
) -> Result<(), Box<dyn Error>> {
    let config = ::aws_config::load_from_env().await;
    let dynamodb_client = dynamodb_sdk_client::new(&config);
    let table_name = envmnt::get_or_panic("TABLE_NAME").to_string();

    // match dynamodb_client
    //     .get_item()
    //     .table_name(&table_name)
    //     .send()
    //     .await?
}
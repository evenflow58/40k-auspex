#![allow(dead_code)]

use serde_derive::{Deserialize, Serialize};
use serde_json::json;
use log::info;
use lambda_runtime::{run, service_fn, LambdaEvent, Error};
use reqwest;

mod policy_builder;
use policy_builder::{PolicyBuilder, APIGatewayCustomAuthorizerPolicy};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct APIGatewayCustomAuthorizerRequest {
    #[serde(rename = "type")]
    _type: String,
    authorization_token: String,
    method_arn: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct APIGatewayCustomAuthorizerResponse {
    principal_id: String,
    policy_document: APIGatewayCustomAuthorizerPolicy,
    context: serde_json::Value,
}

async fn function_handler(
    event: LambdaEvent<APIGatewayCustomAuthorizerRequest>
) -> Result<APIGatewayCustomAuthorizerResponse, Error> {
    info!("Event: {:#?}", event);

    // Make a get request to https://oauth2.googleapis.com/tokeninfo?id_token={token}
    // to validate the token. This should return a GoogleAuthResponse struct.
    let res = reqwest::get(&format!(
            "https://oauth2.googleapis.com/tokeninfo?id_token={}",
            event.payload.authorization_token
        ))
        .await?;
        // .json::<GoogleAuthResponse>()
        // .await?;

    info!("Google response {:#?}", res);

    // let method_arn_array: Vec<&str> = event.payload.method_arn.split(":").collect();
    // let api_gateway_arn_tmp: Vec<&str> = method_arn_array[5].split("/").collect();

    // let policy_builder = PolicyBuilder::new(
    //     method_arn_array[3],
    //     method_arn_array[4],
    //     api_gateway_arn_tmp[0],
    //     api_gateway_arn_tmp[1],
    // );

    // info!("policy_builder: {:#?}", policy_builder);

    // // Make sure the aud is 181396477895-mif6hcekhvhi32up28g49hve07vlvchm.apps.googleusercontent.com
    // // and the iss is https://accounts.google.com
    // if res.aud == "181396477895-mif6hcekhvhi32up28g49hve07vlvchm.apps.googleusercontent.com"
    //     && res.iss == "https://accounts.google.com"
    // {
    //     let response = APIGatewayCustomAuthorizerResponse {
    //         principal_id: res.sub,
    //         policy_document: policy_builder.allow_all_methods().build(),
    //         context: json!({
    //             "email": res.email,
    //             "email_verified": res.email_verified,
    //             "name": res.name,
    //             "picture": res.picture,
    //             "given_name": res.given_name,
    //             "family_name": res.family_name,
    //             "locale": res.locale,
    //         }),
    //     };

    //     info!("Positive response being sent {:#?}", response);
    //     return Ok(response);
    // }

    // let response = APIGatewayCustomAuthorizerResponse {
    //     principal_id: "".to_string(),
    //     policy_document: policy_builder.deny_all_methods().build(),
    //     context: json!({}),
    // };
    // info!("Negative response being sent {:#?}", response);
    // return Ok(response);

    panic!("Ooops");
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    simple_logger::init_with_level(log::Level::Info)?;
    run(service_fn(function_handler)).await
}

// Create a struct for the Google auth response
#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
struct GoogleAuthResponse {
    aud: String,
    exp: i64,
    iat: i64,
    iss: String,
    sub: String,
    email: String,
    email_verified: String,
    name: String,
    picture: String,
    given_name: String,
    family_name: String,
    locale: String,
}
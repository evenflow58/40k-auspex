use aws_lambda_events::event::apigw::ApiGatewayProxyRequest;
use lambda_runtime::LambdaEvent;

pub trait ApiContext {
    fn get_email(&self) -> String;
}

impl ApiContext for LambdaEvent<ApiGatewayProxyRequest> {
    fn get_email(&self) -> String {
        self
        .payload
        .request_context
        .authorizer
        .get("email")
        .unwrap()
        .to_string()
        .replace("\"", "")
    }
}
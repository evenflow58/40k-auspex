use serde::{Deserialize, Serialize};
use std::fmt::{Debug};
use log::info;

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct APIGatewayCustomAuthorizerPolicy {
    Version: String,
    Statement: Vec<IAMPolicyStatement>,
}

static POLICY_VERSION: &str = "2012-10-17"; // override if necessary

#[derive(Serialize, Deserialize, Debug)]
pub enum Method {
    #[serde(rename = "GET")]
    Get,
    #[serde(rename = "POST")]
    Post,
    #[serde(rename = "*PUT")]
    Put,
    #[serde(rename = "DELETE")]
    Delete,
    #[serde(rename = "PATCH")]
    Patch,
    #[serde(rename = "HEAD")]
    Head,
    #[serde(rename = "OPTIONS")]
    Options,
    #[serde(rename = "*")]
    All,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Effect {
    Allow,
    Deny,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
struct IAMPolicyStatement {
    Action: Vec<String>,
    Effect: Effect,
    Resource: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PolicyBuilder {
    region: String,
    aws_account_id: String,
    rest_api_id: String,
    stage: String,
    policy: APIGatewayCustomAuthorizerPolicy,
}

impl PolicyBuilder {
    pub fn new(
        region: &str,
        account_id: &str,
        api_id: &str,
        stage: &str,
    ) -> PolicyBuilder {
        Self {
            region: region.to_string(),
            aws_account_id: account_id.to_string(),
            rest_api_id: api_id.to_string(),
            stage: stage.to_string(),
            policy: APIGatewayCustomAuthorizerPolicy {
                Version: POLICY_VERSION.to_string(),
                Statement: vec![],
            },
        }
    }

    pub fn add_method<T: Into<String> + std::fmt::Debug>(
        mut self,
        effect: Effect,
        method: Method,
        resource: T,
    ) -> Self {
        info!("method: {:?}", method);
        info!("resource: {:?}", resource);
        let resource_arn = format!(
            "arn:aws:execute-api:{}:{}:{}/{}/{}/{}",
            &self.region,
            &self.aws_account_id,
            &self.rest_api_id,
            &self.stage,
            "*",
            "*"
            // serde_json::to_string(&method).unwrap(),
            // resource.into().trim_start_matches("/")
        );

        let stmt = IAMPolicyStatement {
            Effect: effect,
            Action: vec!["execute-api:Invoke".to_string()],
            Resource: vec![resource_arn],
        };

        self.policy.Statement.push(stmt);
        self
    }

    pub fn allow_all_methods(self) -> Self {
        self.add_method(Effect::Allow, Method::All, "*")
    }

    pub fn deny_all_methods(self) -> Self {
        self.add_method(Effect::Deny, Method::All, "*")
    }

    // Creates and executes a new child thread.
    pub fn build(self) -> APIGatewayCustomAuthorizerPolicy {
        self.policy
    }
}
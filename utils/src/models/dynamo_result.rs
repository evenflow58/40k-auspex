use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DynamoResult<T> {
    pub id: String,
    pub data: T,
}
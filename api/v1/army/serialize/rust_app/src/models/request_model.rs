use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RequestModel {
    pub name: String,
    pub army: String,
}

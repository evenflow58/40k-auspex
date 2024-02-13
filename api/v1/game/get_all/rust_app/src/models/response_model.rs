use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ResponseModel {
    pub id: String,
    pub name: String,
}

impl ResponseModel {
    pub fn new(id: String, name: String) -> Self {
        ResponseModel { id: id, name: name }
    }
}

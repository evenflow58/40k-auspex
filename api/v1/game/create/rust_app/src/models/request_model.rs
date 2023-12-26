use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RequestModel {
    pub players_in_game: Vec<String>,
    pub date: DateTime<Utc>,
    pub name: String, 
}
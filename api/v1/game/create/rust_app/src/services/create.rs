use std::error::Error;
use tracing::info;
use chrono::{DateTime, Utc};

pub async fn create(
    players_in_game: Vec<String>,
    name: String,
) -> Result<String, Box<dyn Error>> {
    // match save_army_list(id, user_id.to_string(), name.to_string(), army_list).await {
    //     Ok(id) => Ok(id),
    //     Err(err) => panic!("{}", err),
    // }

    Ok("dummy string".to_string())
}

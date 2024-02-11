use std::error::Error;

use crate::data::army;
use utils::models::{army::Army, army_list::ArmyList};

pub async fn get_armies() -> Result<Vec<Army>, Box<dyn Error>> {
    match army::get_armies().await {
        Ok(armies) => Ok(armies),
        Err(err) => panic!("{:?}", err),
    }
}

pub async fn save_army_list(
    id: Option<&String>,
    user_id: String,
    name: String,
    data: ArmyList,
) -> Result<String, Box<dyn Error>> {
    match army::save_army_list(id, user_id, name, data).await {
        Ok(id) => Ok(id),
        Err(err) => panic!("{:?}", err),
    }
}

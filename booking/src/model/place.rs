use sqlx::FromRow;
use serde::{Deserialize, Serialize};

#[derive(FromRow, Serialize, Debug)]
pub struct Place {
    pub id: i32,
    pub name: String,
    pub capacity: i32,
    pub is_open: bool,
}


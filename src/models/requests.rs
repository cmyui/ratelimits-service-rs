
use chrono::prelude::*;
use serde::{Serialize, Deserialize};
use sqlx;


#[derive(Serialize, sqlx::FromRow)]
pub struct Request {
    pub id: i32,
    pub api_key_id: i32,
    pub method: String,
    pub path: String,
    pub created_at: NaiveDateTime,
}

#[derive(Deserialize)]
pub struct RequestInput {
    pub api_key_id: i32,
    pub method: String,
    pub path: String,
}

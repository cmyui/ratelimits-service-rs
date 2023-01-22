
use chrono::prelude::*;
use sqlx;
use serde::{Serialize, Deserialize};


#[derive(Serialize, sqlx::FromRow)]
pub struct ApiKey {
    pub id: i32,
    pub name: String,
    pub api_key: String,
    pub status: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize)]
pub struct ApiKeyInput {
    pub name: String,
    pub api_key: String,
}

#[derive(Deserialize)]
pub struct ApiKeyUpdate {
    pub name: Option<String>,
    pub api_key: Option<String>,
}

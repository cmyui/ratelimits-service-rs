
use chrono::prelude::*;
use serde::{Serialize, Deserialize};
use sqlx;


#[derive(Serialize, sqlx::FromRow)]
pub struct Ratelimit {
    pub id: i32,
    pub api_key_id: i32,
    pub period: i32,
    pub limit: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize)]
pub struct RatelimitInput {
    pub api_key_id: i32,
    pub period: i32,
    pub limit: i32,
}

#[derive(Deserialize)]
pub struct RatelimitUpdate {
    pub period: Option<i32>,
    pub limit: Option<i32>,
}

use crate::state::AppState;
use crate::models::ratelimits::Ratelimit;
use crate::repositories;
use std::sync::Arc;


pub async fn get_ratelimits(
    state: &Arc<AppState>,
) -> Vec<Ratelimit> {
    let data = repositories::ratelimits::get_ratelimits(&state).await;
    data
}

pub async fn get_ratelimit(
    state: Arc<AppState>,
    ratelimit_id: i32,
) -> Option<Ratelimit> {
    let data = repositories::ratelimits::get_ratelimit(&state, ratelimit_id).await;
    data
}

pub async fn create_ratelimit(
    state: Arc<AppState>,
    api_key_id: i32,
    period: i32,
    limit: i32,
) -> Ratelimit {
    let last_insert_id = repositories::ratelimits::create_ratelimit(&state, api_key_id, period, limit).await;
    let data = repositories::ratelimits::get_ratelimit(&state, last_insert_id as i32).await;
    data.unwrap()
}

pub async fn update_ratelimit(
    state: Arc<AppState>,
    ratelimit_id: i32,
    period: Option<i32>,
    limit: Option<i32>,
) -> Option<Ratelimit> {
    let data = repositories::ratelimits::get_ratelimit(&state, ratelimit_id).await;
    if data.is_none() {
        return None;
    }
    repositories::ratelimits::update_ratelimit(&state, ratelimit_id, period, limit).await;
    let data = repositories::ratelimits::get_ratelimit(&state, ratelimit_id).await;
    Some(data.unwrap())
}

pub async fn delete_ratelimit(
    state: Arc<AppState>,
    ratelimit_id: i32,
) -> Option<Ratelimit> {
    let data = repositories::ratelimits::get_ratelimit(&state, ratelimit_id).await;
    if data.is_none() {
        return None;
    }
    repositories::ratelimits::delete_ratelimit(&state, ratelimit_id).await;
    Some(data.unwrap())
}

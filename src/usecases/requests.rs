use crate::state::AppState;
use crate::models::requests::Request;
use crate::repositories;
use std::sync::Arc;


pub async fn get_requests(
    state: &Arc<AppState>,
) -> Vec<Request> {
    let data = repositories::requests::get_requests(&state).await;
    data
}

pub async fn get_request(
    state: Arc<AppState>,
    request_id: i32,
) -> Option<Request> {
    let data = repositories::requests::get_request(&state, request_id).await;
    data
}

pub async fn create_request(
    state: Arc<AppState>,
    api_key_id: i32,
    method: String,
    path: String,
) -> Request {
    let last_insert_id = repositories::requests::create_request(&state, api_key_id, method, path).await;
    let data = repositories::requests::get_request(&state, last_insert_id as i32).await;
    data.unwrap()
}

pub async fn update_request(
    state: Arc<AppState>,
    request_id: i32,
    method: Option<String>,
    path: Option<String>,
) -> Option<Request> {
    let data = repositories::requests::get_request(&state, request_id).await;
    if data.is_none() {
        return None;
    }
    repositories::requests::update_request(&state, request_id, method, path).await;
    let data = repositories::requests::get_request(&state, request_id).await;
    Some(data.unwrap())
}

pub async fn delete_request(
    state: Arc<AppState>,
    request_id: i32,
) -> Option<Request> {
    let data = repositories::requests::get_request(&state, request_id).await;
    if data.is_none() {
        return None;
    }
    repositories::requests::delete_request(&state, request_id).await;
    Some(data.unwrap())
}

use crate::state::AppState;
use crate::models::api_keys::ApiKey;
use crate::repositories;
use std::sync::Arc;


pub async fn get_api_keys(
    state: &Arc<AppState>,
) -> Vec<ApiKey> {
    let data = repositories::api_keys::get_api_keys(&state).await;
    data
}

pub async fn get_api_key(
    state: Arc<AppState>,
    api_key_id: u32,
) -> Option<ApiKey> {
    let data = repositories::api_keys::get_api_key(state, api_key_id).await;
    data
}

pub async fn create_api_key(
    state: Arc<AppState>,
    name: String,
    api_key: String,
) -> ApiKey {
    let data = repositories::api_keys::create_api_key(state, name, api_key).await;
    data
}

pub async fn update_api_key(
    state: Arc<AppState>,
    api_key_id: u32,
    name: Option<String>,
    api_key: Option<String>,
) -> ApiKey {
    let data = repositories::api_keys::update_api_key(state, api_key_id, name, api_key).await;
    data
}

pub async fn delete_api_key(
    state: Arc<AppState>,
    api_key_id: u32,
) -> ApiKey {
    let data = repositories::api_keys::delete_api_key(state, api_key_id).await;
    data
}

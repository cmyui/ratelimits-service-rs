use crate::state::AppState;
use crate::models::api_keys::{ApiKey, ApiKeyInput, ApiKeyUpdate};
use crate::usecases;

use std::sync::Arc;
use axum::{
    http::StatusCode,
    Json,
    extract,
};

pub async fn get_api_keys(
    extract::State(state): extract::State<Arc<AppState>>,
) -> Json<Vec<ApiKey>> {
    let data = usecases::api_keys::get_api_keys(&state).await;
    Json(data)
}

pub async fn get_api_key(
    extract::State(state): extract::State<Arc<AppState>>,
    extract::Path(api_key_id): extract::Path<u32>,
) -> Result<Json<ApiKey>, StatusCode> {
    match usecases::api_keys::get_api_key(state, api_key_id).await {
        Some(val) => Ok(Json(val)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn create_api_key(
    extract::State(state): extract::State<Arc<AppState>>,
    extract::Json(args): extract::Json<ApiKeyInput>,
) -> Json<ApiKey> {
    let data = usecases::api_keys::create_api_key(state, args.name, args.api_key).await;
    Json(data)
}

pub async fn update_api_key(
    extract::State(state): extract::State<Arc<AppState>>,
    extract::Path(api_key_id): extract::Path<u32>,
    extract::Json(args): extract::Json<ApiKeyUpdate>,
) -> Result<Json<ApiKey>, StatusCode> {
    match usecases::api_keys::update_api_key(state, api_key_id, args.name, args.api_key).await {
        Some(val) => Ok(Json(val)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn delete_api_key(
    extract::State(state): extract::State<Arc<AppState>>,
    extract::Path(api_key_id): extract::Path<u32>,
) -> Result<Json<ApiKey>, StatusCode> {
    match usecases::api_keys::delete_api_key(state, api_key_id).await {
        Some(val) => Ok(Json(val)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

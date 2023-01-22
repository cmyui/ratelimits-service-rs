use crate::state::AppState;
use crate::models::requests::{Request, RequestInput};
use crate::usecases;

use std::sync::Arc;
use axum::{
    http::StatusCode,
    Json,
    extract,
};

pub async fn get_requests(
    extract::State(state): extract::State<Arc<AppState>>,
) -> Json<Vec<Request>> {
    let requests = usecases::requests::get_requests(
        &state,
    ).await;
    Json(requests)
}

pub async fn get_request(
    extract::State(state): extract::State<Arc<AppState>>,
    extract::Path(request_id): extract::Path<i32>,
) -> Result<Json<Request>, StatusCode> {
    match usecases::requests::get_request(
        state,
        request_id,
    ).await {
        Some(request) => Ok(Json(request)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn create_request(
    extract::State(state): extract::State<Arc<AppState>>,
    extract::Json(args): extract::Json<RequestInput>,
) -> Json<Request> {
    let request = usecases::requests::create_request(
        state,
        args.api_key_id,
        args.method,
        args.path,
    ).await;
    Json(request)
}

pub async fn delete_request(
    extract::State(state): extract::State<Arc<AppState>>,
    extract::Path(request_id): extract::Path<i32>,
) -> Result<Json<Request>, StatusCode> {
    match usecases::requests::delete_request(
        state,
        request_id,
    ).await {
        Some(request) => Ok(Json(request)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

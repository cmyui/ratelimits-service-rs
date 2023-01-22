use crate::state::AppState;
use crate::models::ratelimits::{Ratelimit, RatelimitInput, RatelimitUpdate};
use crate::usecases;

use std::sync::Arc;
use axum::{
    http::StatusCode,
    Json,
    extract,
};

pub async fn get_ratelimits(
    extract::State(state): extract::State<Arc<AppState>>,
) -> Json<Vec<Ratelimit>> {
    let ratelimits = usecases::ratelimits::get_ratelimits(
        &state,
    ).await;
    Json(ratelimits)
}

pub async fn get_ratelimit(
    extract::State(state): extract::State<Arc<AppState>>,
    extract::Path(ratelimit_id): extract::Path<i32>,
) -> Result<Json<Ratelimit>, StatusCode> {
    match usecases::ratelimits::get_ratelimit(
        state,
        ratelimit_id,
    ).await {
        Some(ratelimit) => Ok(Json(ratelimit)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn create_ratelimit(
    extract::State(state): extract::State<Arc<AppState>>,
    extract::Json(args): extract::Json<RatelimitInput>,
) -> Json<Ratelimit> {
    let ratelimit = usecases::ratelimits::create_ratelimit(
        state,
        args.api_key_id,
        args.period,
        args.limit,
    ).await;
    Json(ratelimit)
}

pub async fn update_ratelimit(
    extract::State(state): extract::State<Arc<AppState>>,
    extract::Path(ratelimit_id): extract::Path<i32>,
    extract::Json(args): extract::Json<RatelimitUpdate>,
) -> Result<Json<Ratelimit>, StatusCode> {
    match usecases::ratelimits::update_ratelimit(
        state,
        ratelimit_id,
        args.period,
        args.limit,
    ).await {
        Some(ratelimit) => Ok(Json(ratelimit)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn delete_ratelimit(
    extract::State(state): extract::State<Arc<AppState>>,
    extract::Path(ratelimit_id): extract::Path<i32>,
) -> Result<Json<Ratelimit>, StatusCode> {
    match usecases::ratelimits::delete_ratelimit(
        state,
        ratelimit_id,
    ).await {
        Some(ratelimit) => Ok(Json(ratelimit)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

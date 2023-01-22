pub mod state;
pub mod repositories;
pub mod models;
pub mod usecases;
pub mod controllers;

use std::{net::SocketAddr, str::FromStr};
use std::env;
use std::sync::Arc;
use std::path::Path;

use sqlx::mysql::MySqlPoolOptions;
use sqlx::migrate::Migrator;
use axum::{
    routing::{get, post, patch, delete},
    Router,
};
use dotenv::dotenv;

// TODO: openapi support

#[tokio::main]
async fn main() {
    dotenv().ok();

    let pool = match MySqlPoolOptions::new()
        // TODO: config pool size
        .min_connections(1)
        .max_connections(10)
        .connect(env::var("DATABASE_URL").unwrap().as_str())
        .await {
            Ok(val) => val,
            Err(error_details) => panic!("{}", error_details),
        };

    let migrator = Migrator::new(Path::new("./migrations")).await.unwrap();
    migrator.run(&pool).await.unwrap();

    let shared_state = Arc::new(state::AppState{pool: pool});

    let app = Router::new()
        .route("/v1/api_keys", get(controllers::api_keys::get_api_keys))
        .route("/v1/api_keys/:id", get(controllers::api_keys::get_api_key))
        .route("/v1/api_keys", post(controllers::api_keys::create_api_key))
        .route("/v1/api_keys/:id", patch(controllers::api_keys::update_api_key))
        .route("/v1/api_keys/:id", delete(controllers::api_keys::delete_api_key))
        .with_state(shared_state);

    let addr = SocketAddr::from_str(
        format!(
            "{}:{}",
            env::var("APP_HOST").unwrap(),
            env::var("APP_PORT").unwrap(),
        ).as_str()
    ).unwrap();

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}


use crate::state::AppState;
use crate::models::api_keys::ApiKey;
use std::sync::Arc;
use sqlx;

static READ_PARAMS: &'static str = "id, name, api_key, status, created_at, updated_at";

pub async fn get_api_keys(
    state: &Arc<AppState>,
) -> Vec<ApiKey> {
    let query = format!("
        SELECT {}
          FROM api_keys
         WHERE status != 'deleted'
    ", READ_PARAMS);

    let data = sqlx::query_as(query.as_str())
        .fetch_all(&state.pool)
        .await
        .unwrap();
    data
}


pub async fn get_api_key(
    state: &Arc<AppState>,
    api_key_id: i32,
) -> Option<ApiKey> {
    let query = format!("
        SELECT {}
          FROM api_keys
         WHERE id = ?
           AND status != 'deleted'
    ", READ_PARAMS);

    let data = sqlx::query_as(query.as_str())
        .bind(api_key_id)
        .fetch_one(&state.pool)
        .await
        .ok();
    data
}

pub async fn get_deleted_api_key(
    state: &Arc<AppState>,
    api_key_id: i32,
) -> Option<ApiKey> {
    let query = format!("
        SELECT {}
          FROM api_keys
         WHERE id = ?
           AND status = 'deleted'
    ", READ_PARAMS);

    let data = sqlx::query_as(query.as_str())
        .bind(api_key_id)
        .fetch_one(&state.pool)
        .await
        .ok();
    data
}

pub async fn create_api_key(
    state: &Arc<AppState>,
    name: String,
    api_key: String,
) -> u64 {
    let query = "
        INSERT INTO api_keys (name, api_key)
             VALUES (?, ?)
    ";

    let insert_result = sqlx::query(query)
        .bind(name)
        .bind(api_key)
        .execute(&state.pool)
        .await
        .unwrap();
    insert_result.last_insert_id()
}

pub async fn update_api_key(
    state: &Arc<AppState>,
    api_key_id: i32,
    name: Option<String>,
    api_key: Option<String>,
) -> () {
    let query = "
        UPDATE api_keys
           SET name = COALESCE(?, name),
               api_key = COALESCE(?, api_key),
               updated_at = NOW()
         WHERE id = ?
           AND status != 'deleted'
    ";
    sqlx::query(query)
        .bind(name)
        .bind(api_key)
        .bind(api_key_id)
        .execute(&state.pool)
        .await
        .unwrap();
}

pub async fn delete_api_key(
    state: &Arc<AppState>,
    api_key_id: i32,
) -> () {
    let query = "
        UPDATE api_keys
           SET status = 'deleted',
               updated_at = NOW()
         WHERE id = ?
           AND status != 'deleted'
    ";
    sqlx::query(query)
        .bind(api_key_id)
        .execute(&state.pool)
        .await
        .unwrap();
}

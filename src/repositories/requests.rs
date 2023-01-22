
use crate::state::AppState;
use crate::models::requests::Request;
use std::sync::Arc;
use sqlx;

static READ_PARAMS: &'static str = "id, api_key_id, method, path, created_at";

pub async fn get_requests(
    state: &Arc<AppState>,
) -> Vec<Request> {
    let query = format!("
        SELECT {}
          FROM requests
    ", READ_PARAMS);

    let data = sqlx::query_as(query.as_str())
        .fetch_all(&state.pool)
        .await
        .unwrap();
    data
}


pub async fn get_request(
    state: &Arc<AppState>,
    request_id: i32,
) -> Option<Request> {
    let query = format!("
        SELECT {}
          FROM requests
         WHERE id = ?
    ", READ_PARAMS);

    let data = sqlx::query_as(query.as_str())
        .bind(request_id)
        .fetch_one(&state.pool)
        .await
        .ok();
    data
}

pub async fn create_request(
    state: &Arc<AppState>,
    api_key_id: i32,
    method: String,
    path: String,
) -> u64 {
    let query = "
        INSERT INTO requests (api_key_id, method, path)
             VALUES (?, ?, ?)
    ";

    let insert_result = sqlx::query(query)
        .bind(api_key_id)
        .bind(method)
        .bind(path)
        .execute(&state.pool)
        .await
        .unwrap();
    insert_result.last_insert_id()
}

pub async fn update_request(
    state: &Arc<AppState>,
    request_id: i32,
    method: Option<String>,
    path: Option<String>,
) -> () {
    let query = "
        UPDATE requests
           SET method = COALESCE(?, method),
               path = COALESCE(?, path),
               updated_at = NOW()
         WHERE id = ?
    ";
    sqlx::query(query)
        .bind(method)
        .bind(path)
        .bind(request_id)
        .execute(&state.pool)
        .await
        .unwrap();
}

pub async fn delete_request(
    state: &Arc<AppState>,
    request_id: i32,
) -> () {
    let query = "
        DELETE FROM requests
              WHERE id = ?
    ";
    sqlx::query(query)
        .bind(request_id)
        .execute(&state.pool)
        .await
        .unwrap();
}

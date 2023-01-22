
use crate::state::AppState;
use crate::models::ratelimits::Ratelimit;
use std::sync::Arc;
use sqlx;

static READ_PARAMS: &'static str = "id, api_key_id, period, `limit`, created_at, updated_at";

pub async fn get_ratelimits(
    state: &Arc<AppState>,
) -> Vec<Ratelimit> {
    let query = format!("
        SELECT {}
          FROM ratelimits
    ", READ_PARAMS);

    let data = sqlx::query_as(query.as_str())
        .fetch_all(&state.pool)
        .await
        .unwrap();
    data
}


pub async fn get_ratelimit(
    state: &Arc<AppState>,
    ratelimit_id: i32,
) -> Option<Ratelimit> {
    let query = format!("
        SELECT {}
          FROM ratelimits
         WHERE id = ?
    ", READ_PARAMS);

    let data = sqlx::query_as(query.as_str())
        .bind(ratelimit_id)
        .fetch_one(&state.pool)
        .await
        .ok();
    data
}

pub async fn create_ratelimit(
    state: &Arc<AppState>,
    api_key_id: i32,
    period: i32,
    limit: i32,
) -> u64 {
    let query = "
        INSERT INTO ratelimits (api_key_id, period, `limit`)
             VALUES (?, ?, ?)
    ";

    let insert_result = sqlx::query(query)
        .bind(api_key_id)
        .bind(period)
        .bind(limit)
        .execute(&state.pool)
        .await
        .unwrap();
    insert_result.last_insert_id()
}

pub async fn update_ratelimit(
    state: &Arc<AppState>,
    ratelimit_id: i32,
    period: Option<i32>,
    limit: Option<i32>,
) -> () {
    let query = "
        UPDATE ratelimits
           SET period = COALESCE(?, period),
               `limit` = COALESCE(?, `limit`),
               updated_at = NOW()
         WHERE id = ?
    ";
    sqlx::query(query)
        .bind(period)
        .bind(limit)
        .bind(ratelimit_id)
        .execute(&state.pool)
        .await
        .unwrap();
}

pub async fn delete_ratelimit(
    state: &Arc<AppState>,
    ratelimit_id: i32,
) -> () {
    let query = "
        DELETE FROM ratelimits
              WHERE id = ?
    ";
    sqlx::query(query)
        .bind(ratelimit_id)
        .execute(&state.pool)
        .await
        .unwrap();
}

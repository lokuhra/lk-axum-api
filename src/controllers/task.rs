use crate::error::AppError;
use crate::models::{auth::Claims, task::Task};
use axum::extract::Query;
use axum::response::IntoResponse;
use axum::{extract::Path, http::StatusCode, Extension, Json};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::PgPool;

#[derive(Debug, Serialize, Deserialize)]
pub struct PaginationParams {
    page: Option<i32>,
    limit: Option<i32>,
}

pub async fn tasks(
    _claims: Claims,
    Extension(pool): Extension<PgPool>,
    query_params: Query<PaginationParams>,
) -> impl IntoResponse {
    let page = query_params.page.unwrap_or(1);
    let limit = query_params.limit.unwrap_or(10);
    let offset = (page - 1) * limit;

    let sql = format!("SELECT * FROM tasks3 LIMIT {} OFFSET {}", limit, offset);

    let result: Result<Vec<Task>, sqlx::Error> =
        sqlx::query_as::<_, Task>(&sql).fetch_all(&pool).await;

    match result {
        Ok(tasks) => (StatusCode::OK, Json(tasks)),
        Err(err) => {
            tracing::error!("error retrieving tasks: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(Vec::<Task>::new()))
        }
    }
}

pub async fn task(
    Path(id): Path<String>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Value>, AppError> {
    let id_i64 = match id.parse::<i64>() {
        Ok(id) => id,
        Err(_) => return Err(AppError::TaskFormatInvalid),
    };

    let sql = "SELECT * FROM tasks3 where id=$1".to_string();

    let result: Result<Task, sqlx::Error> =
        sqlx::query_as(&sql).bind(id_i64).fetch_one(&pool).await;

    match result {
        Ok(task) => Ok(Json(serde_json::json!(task))),
        Err(_) => Err(AppError::TaskNotExist),
    }
}

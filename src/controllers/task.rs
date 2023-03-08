use crate::models::{auth::Claims, task::Task};
use axum::extract::Path;
use axum::{http::StatusCode, Extension, Json};
use sqlx::PgPool;

use axum::response::IntoResponse;

pub async fn tasks(_claims: Claims, Extension(pool): Extension<PgPool>) -> impl IntoResponse {
    let sql = "SELECT * FROM tasks3".to_string();

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

pub async fn task(Path(id): Path<i64>, Extension(pool): Extension<PgPool>) -> impl IntoResponse {
    let sql = "SELECT * FROM tasks3 where id=$1".to_string();

    let result: Result<Task, sqlx::Error> = sqlx::query_as(&sql).bind(id).fetch_one(&pool).await;

    match result {
        Ok(task) => (StatusCode::OK, Json(task)),
        Err(err) => {
            tracing::error!("could not find task with id: {:?} error: {:?}", id, err);
            (
                StatusCode::NOT_FOUND,
                Json(Task {
                    id,
                    task: "err".to_string(),
                }),
            )
        }
    }
}

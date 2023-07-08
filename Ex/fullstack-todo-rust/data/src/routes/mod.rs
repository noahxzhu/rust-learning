mod create_task;
use axum::{routing::post, Extension, Router};
use create_task::create_task;
use sea_orm::DatabaseConnection;

pub async fn create_routes(database: DatabaseConnection) -> Router {
    Router::new()
        .route("/tasks", post(create_task))
        .layer(Extension(database))
}

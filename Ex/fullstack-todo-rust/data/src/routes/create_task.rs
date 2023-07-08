use axum::Extension;
use sea_orm::DatabaseConnection;



use crate::::tasks;

pub async fn create_task(Extension(database): Extension<DatabaseConnection>) {
    let new_task = tasks::ActiveModel {
        ..Default::default()
    };
}

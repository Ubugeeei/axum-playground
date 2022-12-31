use std::sync::Arc;

use axum::{extract::Extension, http::StatusCode, response::IntoResponse, Json};

use crate::di::{interface::ModulesInterface, modules::Modules};

use super::{
    dto::{TodoCreateInput, TodoUpdateInput},
    interface::TodoRepositoryInterface,
};

pub async fn create_todo(
    Json(payload): Json<TodoCreateInput>,
    Extension(modules): Extension<Arc<Modules>>,
) -> impl IntoResponse {
    let user = modules.todo_repository().create(payload).unwrap();
    (StatusCode::CREATED, Json(user))
}

pub async fn find_todo_by_id(
    Json(id): Json<u64>,
    Extension(modules): Extension<Arc<Modules>>,
) -> impl IntoResponse {
    let user = modules.todo_repository().find_by_id(id).unwrap();
    (StatusCode::OK, Json(user))
}

pub async fn update_todo(
    Json(payload): Json<TodoUpdateInput>,
    Extension(modules): Extension<Arc<Modules>>,
) -> impl IntoResponse {
    let user = modules.todo_repository().update(payload).unwrap();
    (StatusCode::OK, Json(user))
}

pub async fn delete_todo(
    Json(id): Json<u64>,
    Extension(modules): Extension<Arc<Modules>>,
) -> impl IntoResponse {
    modules.todo_repository().delete(id).unwrap();
    (StatusCode::NO_CONTENT, Json(()))
}

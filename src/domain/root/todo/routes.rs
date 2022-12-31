use axum::{
    routing::{get, post},
    Router,
};

use super::controller::{create_todo, delete_todo, find_todo_by_id, update_todo};

pub fn create_user_route() -> Router {
    Router::new()
        .route("/create", post(create_todo))
        .route("/find", get(find_todo_by_id))
        .route("/update", post(update_todo))
        .route("/delete", post(delete_todo))
}

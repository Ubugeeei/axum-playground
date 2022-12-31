use axum::{routing::get, Router};

use super::controller::hello;

pub fn create_root_route() -> Router {
    Router::new().route("/", get(hello))
}

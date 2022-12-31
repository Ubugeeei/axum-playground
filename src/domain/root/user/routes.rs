use axum::{routing::post, Router};

pub fn create_user_route() -> Router {
    Router::new().route(
        "/create",
        post(crate::domain::root::user::controller::create_user),
    )
}

use std::net::SocketAddr;

use axum::{extract::Extension, Router};

use crate::{
    di::modules::Modules,
    domain::root::{
        route::create_root_route, todo::routes::create_todo_route, user::routes::create_user_route,
    },
};

mod di;
mod domain;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);

    let modules = Modules::new().await;

    let app = Router::new()
        .nest("/", create_root_route())
        .nest("/users", create_user_route())
        .nest("/todos", create_todo_route())
        .layer(Extension(modules));

    // boot
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

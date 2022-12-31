use std::net::SocketAddr;

use axum::{extract::Extension, Router};

mod di;
mod domain;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);

    let modules = di::modules::Modules::new().await;

    let app = Router::new()
        .nest("/", domain::root::route::create_root_route())
        .nest("/user", domain::root::user::routes::create_user_route())
        .nest("/todo", domain::root::todo::routes::create_user_route())
        .layer(Extension(modules));

    // boot
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

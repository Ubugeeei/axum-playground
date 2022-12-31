use std::sync::Arc;

use axum::{extract::Extension, http::StatusCode, response::IntoResponse, Json};

use crate::{
    di::{interface::ModulesInterface, modules::Modules},
    domain::root::user::{dto::CreateUser, interface::UserRepositoryInterface},
};

pub(crate) async fn create_user(
    Json(payload): Json<CreateUser>,
    Extension(modules): Extension<Arc<Modules>>,
) -> impl IntoResponse {
    let user = modules.user_repository().create(payload).unwrap();
    (StatusCode::CREATED, Json(user))
}

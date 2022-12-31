use serde::Serialize;

#[derive(Serialize)]
pub struct Todo {
    pub id: u64,
    pub user_id: u64,
    pub title: String,
}

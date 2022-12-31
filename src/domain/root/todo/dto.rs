use serde::Deserialize;

#[derive(Deserialize)]
pub struct TodoCreateInput {
    pub user_id: u64,
    pub title: String,
}

#[derive(Deserialize)]
pub struct TodoUpdateInput {
    pub id: u64,
    pub user_id: u64,
    pub title: String,
}

use crate::domain::root::{todo::repository::TodoRepository, user::repository::UserRepository};

use super::interface::ModulesInterface;

#[derive(Clone)]
pub struct Modules {
    pub user_repository: UserRepository,
    pub todo_repository: TodoRepository,
}

impl Modules {
    pub async fn new() -> Modules {
        Modules {
            user_repository: UserRepository,
            todo_repository: TodoRepository,
        }
    }
}

impl ModulesInterface for Modules {
    fn user_repository(&self) -> &UserRepository {
        &self.user_repository
    }

    fn todo_repository(&self) -> &TodoRepository {
        &self.todo_repository
    }
}

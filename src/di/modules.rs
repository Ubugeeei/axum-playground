use crate::domain::root::user::repository::UserRepository;

use super::interface::ModulesInterface;

#[derive(Clone)]
pub struct Modules {
    pub user_repository: UserRepository,
}

impl Modules {
    pub async fn new() -> Modules {
        Modules {
            user_repository: UserRepository,
        }
    }
}

impl ModulesInterface for Modules {
    fn user_repository(&self) -> &UserRepository {
        &self.user_repository
    }
}

use crate::domain::root::{todo::repository::TodoRepository, user::repository::UserRepository};

pub trait ModulesInterface {
    fn user_repository(&self) -> &UserRepository;
    fn todo_repository(&self) -> &TodoRepository;
}

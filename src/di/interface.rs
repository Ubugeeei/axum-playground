use crate::domain::root::user::repository::UserRepository;

pub trait ModulesInterface {
    fn user_repository(&self) -> &UserRepository;
}

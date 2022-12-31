use super::{dto::CreateUser, entity::User};

pub trait UserRepositoryInterface {
    fn create(&self, user: CreateUser) -> Result<User, ()>;
}

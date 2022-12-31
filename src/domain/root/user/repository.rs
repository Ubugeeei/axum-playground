use super::{dto::CreateUser, entity::User, interface::UserRepositoryInterface};

#[derive(Clone)]
pub struct UserRepository;

impl UserRepositoryInterface for UserRepository {
    fn create(&self, user: CreateUser) -> Result<User, ()> {
        Ok(User {
            id: 1, // TODO: Generate ID
            username: user.username,
        })
    }
}

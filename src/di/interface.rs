use crate::domain::root::{
    todo::interface::TodoRepositoryInterface, user::interface::UserRepositoryInterface,
};

pub trait ModulesInterface<T: UserRepositoryInterface, U: TodoRepositoryInterface> {
    fn user_repository(&self) -> &T;
    fn todo_repository(&self) -> &U;
}

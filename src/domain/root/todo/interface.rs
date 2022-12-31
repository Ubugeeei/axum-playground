use super::{
    dto::{TodoCreateInput, TodoUpdateInput},
    entity::Todo,
};

pub trait TodoRepositoryInterface {
    fn create(&self, user: TodoCreateInput) -> Result<Todo, ()>;
    fn find_by_id(&self, id: u64) -> Result<Todo, ()>;
    fn update(&self, user: TodoUpdateInput) -> Result<Todo, ()>;
    fn delete(&self, id: u64) -> Result<(), ()>;
}

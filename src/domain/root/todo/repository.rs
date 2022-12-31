use super::{
    dto::{TodoCreateInput, TodoUpdateInput},
    entity::Todo,
    interface::TodoRepositoryInterface,
};

#[derive(Clone)]
pub struct TodoRepository;

impl TodoRepositoryInterface for TodoRepository {
    fn create(&self, todo: TodoCreateInput) -> Result<Todo, ()> {
        Ok(Todo {
            id: 1,
            user_id: todo.user_id,
            title: todo.title,
        })
    }

    fn find_by_id(&self, id: u64) -> Result<Todo, ()> {
        Ok(Todo {
            id: id as u64,
            user_id: 1,
            title: "title".to_string(),
        })
    }

    fn update(&self, todo: TodoUpdateInput) -> Result<Todo, ()> {
        let mut todo = self.find_by_id(todo.id).unwrap();
        Ok({
            todo.title = todo.title;
            todo
        })
    }

    fn delete(&self, _id: u64) -> Result<(), ()> {
        Ok(())
    }
}

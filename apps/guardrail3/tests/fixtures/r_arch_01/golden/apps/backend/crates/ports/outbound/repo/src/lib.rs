use backend_domain_types::Task;

pub trait TaskRepo {
    fn list_inbox_tasks(&self) -> Vec<Task>;
    fn replace_schedule(&mut self, tasks: Vec<Task>);
}

use backend_domain_types::{Task, TaskKind, TaskStatus};
use backend_ports_outbound_repo::TaskRepo;

#[derive(Debug, Default)]
pub struct PostgresTaskRepo {
    household_slug: String,
    inbox: Vec<Task>,
    scheduled: Vec<Task>,
}

impl PostgresTaskRepo {
    pub fn seeded(household_slug: impl Into<String>) -> Self {
        let mut inbox = Vec::new();
        if let Ok(task) = Task::try_new("dishwasher", "Empty the dishwasher", TaskKind::Chore, 1) {
            inbox.push(task);
        }
        if let Ok(task) = Task::try_new("groceries", "Order next week's groceries", TaskKind::Errand, 3)
        {
            inbox.push(task.with_notes("Prefer curbside pickup before Wednesday soccer."));
        }
        if let Ok(task) = Task::try_new("forms", "Finish school trip forms", TaskKind::School, 5) {
            inbox.push(task.carryover(2).pinned());
        }

        Self {
            household_slug: household_slug.into(),
            inbox,
            scheduled: Vec::new(),
        }
    }

    pub fn household_slug(&self) -> &str {
        &self.household_slug
    }

    pub fn scheduled_tasks(&self) -> &[Task] {
        &self.scheduled
    }
}

impl TaskRepo for PostgresTaskRepo {
    fn list_inbox_tasks(&self) -> Vec<Task> {
        self.inbox
            .iter()
            .filter(|task| task.status == TaskStatus::Inbox)
            .cloned()
            .collect()
    }

    fn replace_schedule(&mut self, tasks: Vec<Task>) {
        self.scheduled = tasks;
    }
}

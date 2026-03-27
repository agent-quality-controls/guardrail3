use backend_domain_engine::build_weekly_plan;
use backend_domain_types::{PlannerError, WeeklyPlan};
use backend_ports_outbound_repo::TaskRepo;

pub fn plan_inbox_week(repo: &mut dyn TaskRepo) -> Result<WeeklyPlan, PlannerError> {
    let tasks = repo.list_inbox_tasks();
    let plan = build_weekly_plan(tasks)?;
    repo.replace_schedule(plan.tasks.clone());
    Ok(plan)
}

#[cfg(test)]
mod tests {
    use backend_domain_types::{Task, TaskKind};
    use backend_ports_outbound_repo::TaskRepo;

    use super::plan_inbox_week;

    #[derive(Default)]
    struct MemoryRepo {
        inbox: Vec<Task>,
        scheduled: Vec<Task>,
    }

    impl TaskRepo for MemoryRepo {
        fn list_inbox_tasks(&self) -> Vec<Task> {
            self.inbox.clone()
        }

        fn replace_schedule(&mut self, tasks: Vec<Task>) {
            self.scheduled = tasks;
        }
    }

    #[test]
    fn plans_tasks_in_priority_order() {
        let mut repo = MemoryRepo {
            inbox: vec![
                Task::try_new("t2", "Call the plumber", TaskKind::Errand, 4).expect("task"),
                Task::try_new("t1", "Book pediatrician visit", TaskKind::Admin, 2).expect("task"),
            ],
            scheduled: Vec::new(),
        };

        let plan = plan_inbox_week(&mut repo).expect("weekly plan should build");

        assert_eq!(plan.focus_points, 6);
        assert_eq!(repo.scheduled.len(), 2);
        assert_eq!(repo.scheduled[0].title, "Book pediatrician visit");
    }

    #[test]
    fn keeps_non_pinned_work_in_overflow_once_focus_budget_is_full() {
        let mut repo = MemoryRepo {
            inbox: vec![
                Task::try_new("forms", "Finish school forms", TaskKind::School, 5)
                    .expect("task")
                    .pinned(),
                Task::try_new("groceries", "Order groceries", TaskKind::Errand, 3).expect("task"),
                Task::try_new("garage", "Sort garage shelf", TaskKind::Chore, 4).expect("task"),
            ],
            scheduled: Vec::new(),
        };

        let plan = plan_inbox_week(&mut repo).expect("weekly plan should build");

        assert_eq!(plan.focus_points, 8);
        assert_eq!(plan.tasks.len(), 2);
        assert_eq!(plan.overflow_tasks.len(), 1);
        assert_eq!(plan.overflow_tasks[0].title, "Sort garage shelf");
    }
}

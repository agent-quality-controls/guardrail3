use backend_domain_types::{Task, TaskStatus, WeeklyPlan};
use backend_ports_outbound_repo::TaskRepo;

pub fn load_dashboard_snapshot(repo: &dyn TaskRepo) -> WeeklyPlan {
    let tasks = repo.list_inbox_tasks();
    let focus_points = tasks
        .iter()
        .filter(|task| task.status != TaskStatus::Completed)
        .map(|task| task.points)
        .sum();

    WeeklyPlan {
        tasks,
        focus_points,
        overflow_tasks: Vec::new(),
    }
}

pub fn find_overdue_candidates(tasks: &[Task]) -> Vec<Task> {
    tasks
        .iter()
        .filter(|task| {
            task.status == TaskStatus::Scheduled
                && (task.points >= 4 || task.carryover_count >= 2 || task.pinned)
        })
        .cloned()
        .collect()
}

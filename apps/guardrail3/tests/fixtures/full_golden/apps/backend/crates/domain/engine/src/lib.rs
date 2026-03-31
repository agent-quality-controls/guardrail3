use backend_domain_types::{PlannerError, Task, TaskStatus, WeeklyPlan};

const MAX_FOCUS_POINTS: u16 = 8;

pub fn build_weekly_plan(mut tasks: Vec<Task>) -> Result<WeeklyPlan, PlannerError> {
    tasks.sort_by(|left, right| {
        right
            .pinned
            .cmp(&left.pinned)
            .then_with(|| right.carryover_count.cmp(&left.carryover_count))
            .then_with(|| left.points
            .cmp(&right.points)
            .then_with(|| left.title.cmp(&right.title)))
    });

    let mut focus_points: u16 = 0;
    let mut scheduled = Vec::new();
    let mut overflow_tasks = Vec::new();
    for mut task in tasks {
        if task.status != TaskStatus::Inbox {
            scheduled.push(task);
            continue;
        }

        let next_focus_points = focus_points.saturating_add(task.points);
        if task.pinned || next_focus_points <= MAX_FOCUS_POINTS {
            task.status = TaskStatus::Scheduled;
            focus_points = next_focus_points;
            scheduled.push(task);
        } else {
            overflow_tasks.push(task);
        }
    }

    Ok(WeeklyPlan {
        tasks: scheduled,
        focus_points,
        overflow_tasks,
    })
}

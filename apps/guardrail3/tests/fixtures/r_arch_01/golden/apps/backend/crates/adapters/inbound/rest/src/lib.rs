use backend_adapters_outbound_postgres::PostgresTaskRepo;
use backend_adapters_outbound_queue::MemoryPlannerQueue;
use backend_app_commands::plan_inbox_week;
use backend_app_queries::{find_overdue_candidates, load_dashboard_snapshot};
use backend_domain_types::PlannerError;
use backend_ports_inbound_api::{DashboardSnapshot, PlanWeekRequest};
use backend_ports_outbound_events::{PlannerEvents, SchedulePublishedEvent};

pub fn handle_dashboard(request: &PlanWeekRequest) -> DashboardSnapshot {
    let repo = PostgresTaskRepo::seeded(&request.household_slug);
    let plan = load_dashboard_snapshot(&repo);
    let overdue_candidates = find_overdue_candidates(&plan.tasks);

    DashboardSnapshot {
        household_slug: request.household_slug.clone(),
        timezone: request.timezone.clone(),
        plan,
        overdue_candidates,
        service_mode: "snapshot".to_owned(),
    }
}

pub fn handle_plan_week(request: &PlanWeekRequest) -> Result<DashboardSnapshot, PlannerError> {
    let mut repo = PostgresTaskRepo::seeded(&request.household_slug);
    let plan = plan_inbox_week(&mut repo)?;
    let overdue_candidates = find_overdue_candidates(&plan.tasks);

    let mut queue = MemoryPlannerQueue::default();
    queue.publish_schedule_ready(SchedulePublishedEvent {
        household_slug: request.household_slug.clone(),
        planned_tasks: plan.tasks.clone(),
        focus_points: plan.focus_points,
    });

    Ok(DashboardSnapshot {
        household_slug: request.household_slug.clone(),
        timezone: request.timezone.clone(),
        plan,
        overdue_candidates,
        service_mode: "planned".to_owned(),
    })
}

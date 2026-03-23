use backend_domain_types::{Task, WeeklyPlan};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlanWeekRequest {
    pub household_slug: String,
    pub timezone: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DashboardSnapshot {
    pub household_slug: String,
    pub timezone: String,
    pub plan: WeeklyPlan,
    pub overdue_candidates: Vec<Task>,
    pub service_mode: String,
}

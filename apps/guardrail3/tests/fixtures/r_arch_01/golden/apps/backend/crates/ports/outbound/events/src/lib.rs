use backend_domain_types::Task;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchedulePublishedEvent {
    pub household_slug: String,
    pub planned_tasks: Vec<Task>,
    pub focus_points: u16,
}

pub trait PlannerEvents {
    fn publish_schedule_ready(&mut self, event: SchedulePublishedEvent);
}

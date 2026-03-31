use backend_ports_outbound_events::{PlannerEvents, SchedulePublishedEvent};

#[derive(Debug, Default)]
pub struct MemoryPlannerQueue {
    published: Vec<SchedulePublishedEvent>,
}

impl MemoryPlannerQueue {
    pub fn published(&self) -> &[SchedulePublishedEvent] {
        &self.published
    }
}

impl PlannerEvents for MemoryPlannerQueue {
    fn publish_schedule_ready(&mut self, event: SchedulePublishedEvent) {
        self.published.push(event);
    }
}

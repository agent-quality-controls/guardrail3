use backend_adapters_inbound_rest::{handle_dashboard, handle_plan_week};
use backend_ports_inbound_api::PlanWeekRequest;

fn main() {
    let request = PlanWeekRequest {
        household_slug: "rivera-family".to_owned(),
        timezone: "America/Chicago".to_owned(),
    };

    let before = handle_dashboard(&request);
    let Ok(after) = handle_plan_week(&request) else {
        eprintln!("plan week request failed for {}", request.household_slug);
        return;
    };

    println!(
        "dashboard for {} ({}): {} inbox tasks before planning, {} scheduled after planning",
        before.household_slug,
        before.timezone,
        before.plan.tasks.len(),
        after.plan.tasks.len()
    );
}

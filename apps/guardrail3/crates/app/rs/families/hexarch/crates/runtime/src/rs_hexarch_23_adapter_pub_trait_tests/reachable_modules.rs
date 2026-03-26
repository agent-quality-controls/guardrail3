use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_23_adapter_pub_trait as assertions;
use super::{copy_fixture, write_file};

#[test]
fn orphan_adapter_source_file_does_not_count_as_public_trait_surface() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/backend/crates/adapters/outbound/postgres/src/orphan.rs",
        "pub trait OrphanBoundary {}\n",
    );

    let results = super::run_family(tmp.path());
    assertions::assert_no_error(&results, "");
}

#[test]
fn cfg_test_module_public_traits_are_ignored() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/backend/crates/adapters/outbound/postgres/src/lib.rs",
        r#"
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

#[cfg(test)]
mod tests {
    pub trait TestOnlyBoundary {}
}
"#,
    );

    let results = super::run_family(tmp.path());
    assertions::assert_no_error(&results, "");
}

#[test]
fn missing_entrypoint_errors_instead_of_scanning_root_rs_files_as_entrypoints() {
    let tmp = copy_fixture();
    std::fs::remove_file(
        tmp.path()
            .join("apps/backend/crates/adapters/outbound/postgres/src/lib.rs"),
    )
    .expect("remove lib.rs");
    write_file(
        tmp.path(),
        "apps/backend/crates/adapters/outbound/postgres/src/orphan.rs",
        "pub trait OrphanBoundary {}\n",
    );

    let results = super::run_family(tmp.path());
    assertions::assert_error_file_single(
        &results,
        "",
        "apps/backend/crates/adapters/outbound/postgres/src",
    );
    assertions::assert_error_message_contains(&results, "", &["expected src/lib.rs or src/main.rs"]);
}

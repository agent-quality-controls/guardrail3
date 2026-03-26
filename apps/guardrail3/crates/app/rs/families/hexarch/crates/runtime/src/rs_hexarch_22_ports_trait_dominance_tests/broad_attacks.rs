use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_22_ports_trait_dominance as assertions;
use super::{copy_fixture, write_file};

#[test]
fn clean_golden_fixture_stays_clear_for_ports_trait_dominance() {
    let tmp = copy_fixture();

    let results = super::run_family(tmp.path());
    assertions::assert_no_warning(&results, "");
}

#[test]
fn private_trait_in_ports_crate_still_counts_as_impl_heavy() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/backend/crates/ports/outbound/repo/src/lib.rs",
        "trait InternalRepo {}\n\nstruct Repo;\n\nimpl Repo {\n    fn new() -> Self {\n        Self\n    }\n}\n\nimpl InternalRepo for Repo {}\n",
    );

    let results = super::run_family(tmp.path());
    assertions::assert_warning_summary(
        &results,
        "",
        1,
        &["apps/backend/crates/ports/outbound/repo"],
        Some(Some("apps/backend/crates/ports/outbound/repo")),
        Some("Ports crate `backend-ports-outbound-repo` has 2 impl blocks and 0 public traits"),
        None,
        &[],
    );
}

#[test]
fn impls_in_multiple_source_files_are_aggregated() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/backend/crates/ports/outbound/repo/src/lib.rs",
        "mod extra;\n\nuse backend_domain_types::Task;\n\npub trait TaskRepo {\n    fn list_inbox_tasks(&self) -> Vec<Task>;\n    fn replace_schedule(&mut self, tasks: Vec<Task>);\n}\n",
    );
    write_file(
        tmp.path(),
        "apps/backend/crates/ports/outbound/repo/src/extra.rs",
        "pub struct ExtraA;\n\nimpl ExtraA {\n    pub fn new() -> Self {\n        Self\n    }\n}\n\npub struct ExtraB;\n\nimpl ExtraB {\n    pub fn new() -> Self {\n        Self\n    }\n}\n",
    );

    let results = super::run_family(tmp.path());
    assertions::assert_warning_summary(
        &results,
        "",
        1,
        &["apps/backend/crates/ports/outbound/repo"],
        Some(Some("apps/backend/crates/ports/outbound/repo")),
        Some("Ports crate `backend-ports-outbound-repo` has 2 impl blocks and 1 public traits"),
        None,
        &[],
    );
}

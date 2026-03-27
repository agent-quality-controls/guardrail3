use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_22_ports_trait_dominance as assertions;
use super::{copy_fixture, write_file};

#[test]
fn clean_golden_fixture_stays_clear_for_ports_trait_dominance() {
    let tmp = copy_fixture();

    let results = super::run_family(tmp.path());
    assertions::assert_no_warning(&results, "");
}

#[test]
fn private_trait_without_public_behavior_stays_clean() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/backend/crates/ports/outbound/repo/src/lib.rs",
        "trait InternalRepo {}\n\nstruct Repo;\n\nimpl Repo {\n    fn new() -> Self {\n        Self\n    }\n}\n\nimpl InternalRepo for Repo {}\n",
    );

    let results = super::run_family(tmp.path());
    assertions::assert_no_warning(&results, "");
}

#[test]
fn public_inherent_methods_in_traitless_helper_module_warn() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/backend/crates/ports/outbound/repo/src/lib.rs",
        "pub mod extra;\n\nuse backend_domain_types::Task;\n\npub trait TaskRepo {\n    fn list_inbox_tasks(&self) -> Vec<Task>;\n    fn replace_schedule(&mut self, tasks: Vec<Task>);\n}\n",
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
        Some("public inherent method"),
        None,
        &[],
    );
}

#[test]
fn private_helper_methods_in_traitless_module_stay_clean() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/backend/crates/ports/outbound/repo/src/lib.rs",
        "mod extra;\n\nuse backend_domain_types::Task;\n\npub trait TaskRepo {\n    fn list_inbox_tasks(&self) -> Vec<Task>;\n    fn replace_schedule(&mut self, tasks: Vec<Task>);\n}\n",
    );
    write_file(
        tmp.path(),
        "apps/backend/crates/ports/outbound/repo/src/extra.rs",
        "pub struct ExtraA;\n\nimpl ExtraA {\n    fn new() -> Self {\n        Self\n    }\n}\n\nstruct ExtraB;\n\nimpl ExtraB {\n    fn new() -> Self {\n        Self\n    }\n}\n",
    );

    let results = super::run_family(tmp.path());
    assertions::assert_no_warning(&results, "");
}

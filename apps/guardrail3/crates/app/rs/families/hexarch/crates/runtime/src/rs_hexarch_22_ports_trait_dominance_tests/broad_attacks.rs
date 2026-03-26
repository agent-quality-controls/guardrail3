use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_22_ports_trait_dominance as assertions;
use guardrail3_domain_report::Severity;
use crate::test_support::{copy_fixture, write_file};

#[test]
fn clean_golden_fixture_stays_clear_for_ports_trait_dominance() {
    let tmp = copy_fixture();

    let results = assertions::run_family(tmp.path());
    let warnings: Vec<_> = results
        .iter()
        .filter(|result| result.id == "RS-HEXARCH-22")
        .collect();

    assert!(
        warnings.is_empty(),
        "expected the golden fixture to stay clear for RS-HEXARCH-22: {warnings:#?}"
    );
}

#[test]
fn private_trait_in_ports_crate_still_counts_as_impl_heavy() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/backend/crates/ports/outbound/repo/src/lib.rs",
        "trait InternalRepo {}\n\nstruct Repo;\n\nimpl Repo {\n    fn new() -> Self {\n        Self\n    }\n}\n\nimpl InternalRepo for Repo {}\n",
    );

    let results = assertions::run_family(tmp.path());
    let warnings: Vec<_> = results
        .iter()
        .filter(|result| result.id == "RS-HEXARCH-22")
        .collect();

    assert_eq!(
        warnings.len(),
        1,
        "expected one trait-dominance warning for a private-trait ports crate: {warnings:#?}"
    );
    assert_eq!(warnings[0].severity, Severity::Warn);
    assert_eq!(
        warnings[0].file.as_deref(),
        Some("apps/backend/crates/ports/outbound/repo")
    );
    assert!(warnings[0].message.contains(
        "Ports crate `backend-ports-outbound-repo` has 2 impl blocks and 0 public traits"
    ));
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

    let results = assertions::run_family(tmp.path());
    let warnings: Vec<_> = results
        .iter()
        .filter(|result| result.id == "RS-HEXARCH-22")
        .collect();

    assert_eq!(
        warnings.len(),
        1,
        "expected one trait-dominance warning when impl blocks are split across files: {warnings:#?}"
    );
    assert_eq!(warnings[0].severity, Severity::Warn);
    assert_eq!(
        warnings[0].file.as_deref(),
        Some("apps/backend/crates/ports/outbound/repo")
    );
    assert!(warnings[0].message.contains(
        "Ports crate `backend-ports-outbound-repo` has 2 impl blocks and 1 public traits"
    ));
}

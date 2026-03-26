use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_22_ports_trait_dominance as assertions;
use test_support::{copy_fixture, write_file};

#[test]
fn orphan_ports_source_file_does_not_count_toward_trait_dominance() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/backend/crates/ports/outbound/repo/src/orphan.rs",
        r#"
struct OrphanRepo;

impl OrphanRepo {
    fn new() -> Self {
        Self
    }
}

impl OrphanRepo {
    fn list(&self) {}
}
"#,
    );

    let results = assertions::run_family(tmp.path());
    let warnings: Vec<_> = results
        .iter()
        .filter(|result| result.id == "RS-HEXARCH-22")
        .collect();

    assert!(
        warnings.is_empty(),
        "unreachable orphan files should not affect RS-HEXARCH-22: {warnings:#?}"
    );
}

#[test]
fn cfg_test_module_impls_do_not_count_toward_trait_dominance() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/backend/crates/ports/outbound/repo/src/lib.rs",
        r#"
use backend_domain_types::Task;

pub trait TaskRepo {
    fn list_inbox_tasks(&self) -> Vec<Task>;
    fn replace_schedule(&mut self, tasks: Vec<Task>);
}

#[cfg(test)]
mod tests {
    struct TestRepo;

    impl TestRepo {
        fn new() -> Self {
            Self
        }
    }

    impl TestRepo {
        fn list(&self) {}
    }
}
"#,
    );

    let results = assertions::run_family(tmp.path());
    let warnings: Vec<_> = results
        .iter()
        .filter(|result| result.id == "RS-HEXARCH-22")
        .collect();

    assert!(
        warnings.is_empty(),
        "test-only impls should not affect RS-HEXARCH-22: {warnings:#?}"
    );
}

#[test]
fn missing_entrypoint_warns_instead_of_scanning_root_rs_files_as_entrypoints() {
    let tmp = copy_fixture();
    std::fs::remove_file(
        tmp.path()
            .join("apps/backend/crates/ports/outbound/repo/src/lib.rs"),
    )
    .expect("remove lib.rs");
    write_file(
        tmp.path(),
        "apps/backend/crates/ports/outbound/repo/src/orphan.rs",
        r#"
pub trait HiddenPort {
    fn list(&self);
}

struct OrphanRepo;

impl OrphanRepo {
    fn new() -> Self {
        Self
    }
}

impl OrphanRepo {
    fn list(&self) {}
}
"#,
    );

    let results = assertions::run_family(tmp.path());
    let warnings: Vec<_> = results
        .iter()
        .filter(|result| result.id == "RS-HEXARCH-22")
        .collect();

    assert_eq!(
        warnings.len(),
        1,
        "missing entrypoints should fail closed once instead of scanning orphan root files: {warnings:#?}"
    );
    assert_eq!(
        warnings[0].file.as_deref(),
        Some("apps/backend/crates/ports/outbound/repo/src")
    );
    assert!(
        warnings[0]
            .message
            .contains("expected src/lib.rs or src/main.rs"),
        "expected explicit missing-entrypoint source warning: {warnings:#?}"
    );
}

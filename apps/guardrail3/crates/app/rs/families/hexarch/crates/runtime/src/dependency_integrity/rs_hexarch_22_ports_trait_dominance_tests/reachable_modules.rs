use super::{copy_fixture, write_file};
use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_22_ports_trait_dominance as assertions;

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

    let results = super::run_family(tmp.path());
    assertions::assert_no_warning(&results, "");
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

    let results = super::run_family(tmp.path());
    assertions::assert_no_warning(&results, "");
}

#[test]
fn missing_entrypoint_warns_instead_of_scanning_root_rs_files_as_entrypoints() {
    let tmp = copy_fixture();
    std::fs::remove_file(
        tmp.path()
            .join("apps/backend/crates/ports/outbound/repo/src/lib.rs"),
    )
    .expect("failed to remove lib.rs from hexarch fixture");
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

    let results = super::run_family(tmp.path());
    assertions::assert_warning_summary(
        &results,
        "",
        1,
        &["apps/backend/crates/ports/outbound/repo/src"],
        Some(Some("apps/backend/crates/ports/outbound/repo/src")),
        Some("expected src/lib.rs or src/main.rs"),
        None,
        &[],
    );
}

#[test]
fn lib_path_override_is_used_as_ports_entrypoint() {
    let tmp = copy_fixture();
    std::fs::write(
        tmp.path()
            .join("apps/backend/crates/ports/outbound/repo/Cargo.toml"),
        "[package]\nname = \"backend-ports-outbound-repo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[lib]\npath = \"repo.rs\"\n",
    )
    .expect("rewrite ports cargo");
    std::fs::remove_file(
        tmp.path()
            .join("apps/backend/crates/ports/outbound/repo/src/lib.rs"),
    )
    .expect("failed to remove lib.rs from hexarch fixture");
    write_file(
        tmp.path(),
        "apps/backend/crates/ports/outbound/repo/repo.rs",
        r#"
pub trait TaskRepo {
    fn list(&self);
}
"#,
    );

    let results = super::run_family(tmp.path());
    assertions::assert_no_warning(&results, "");
}

#[test]
fn reachable_public_free_function_warns() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/backend/crates/ports/outbound/repo/src/lib.rs",
        r#"
pub trait TaskRepo {
    fn list(&self);
}

pub fn helper() {}
"#,
    );

    let results = super::run_family(tmp.path());
    assertions::assert_warning_summary(
        &results,
        "",
        1,
        &["apps/backend/crates/ports/outbound/repo"],
        Some(Some("apps/backend/crates/ports/outbound/repo")),
        Some("public free function"),
        None,
        &[],
    );
}

#[test]
fn public_items_inside_private_module_do_not_count_as_ports_public_surface() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/backend/crates/ports/outbound/repo/src/lib.rs",
        r#"
mod private_api;

pub trait TaskRepo {
    fn list(&self);
}
"#,
    );
    write_file(
        tmp.path(),
        "apps/backend/crates/ports/outbound/repo/src/private_api.rs",
        r#"
pub fn helper() {}

pub struct InternalRepo;

impl InternalRepo {
    pub fn new() -> Self {
        Self
    }
}
"#,
    );

    let results = super::run_family(tmp.path());
    assertions::assert_no_warning(&results, "");
}

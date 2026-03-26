use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_07_workspace_members_match_crate_dirs as assertions;
use crate::test_support::{copy_fixture, write_file};

#[test]
fn nested_cargo_project_inside_real_leaf_is_not_treated_as_required_workspace_member() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/devctl/crates/domain/types/examples/demo/Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n",
    );
    write_file(
        tmp.path(),
        "apps/devctl/crates/domain/types/examples/demo/src/lib.rs",
        "// demo",
    );

    let results = assertions::run_family(tmp.path());
    assert!(
        assertions::errors_by_id(&results, "RS-HEXARCH-07").is_empty(),
        "{results:#?}"
    );
}

#[test]
fn packages_crates_do_not_enter_rule_07_discovery() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "packages/shared-types/crates/domain/events/Cargo.toml",
        "[package]\nname = \"shared-types-domain-events\"\nversion = \"0.1.0\"\n",
    );
    write_file(
        tmp.path(),
        "packages/shared-types/crates/domain/events/src/lib.rs",
        "// shared types event model",
    );

    let results = assertions::run_family(tmp.path());
    assert!(
        assertions::errors_by_id(&results, "RS-HEXARCH-07").is_empty(),
        "{results:#?}"
    );
}

#[test]
fn non_rust_app_lookalikes_do_not_enter_rule_07_discovery() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/admin/crates/domain/events/Cargo.toml",
        "[package]\nname = \"admin-domain-events\"\nversion = \"0.1.0\"\n",
    );
    write_file(
        tmp.path(),
        "apps/admin/crates/domain/events/src/lib.rs",
        "// admin events",
    );

    let results = assertions::run_family(tmp.path());
    assert!(
        assertions::errors_by_id(&results, "RS-HEXARCH-07").is_empty(),
        "{results:#?}"
    );
}

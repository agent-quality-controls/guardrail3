use super::super::super::test_support::{assert_no_error, copy_fixture, run_family, write_file};

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

    let results = run_family(tmp.path());
    assert_no_error(&results, "RS-HEXARCH-07");
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

    let results = run_family(tmp.path());
    assert_no_error(&results, "RS-HEXARCH-07");
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

    let results = run_family(tmp.path());
    assert_no_error(&results, "RS-HEXARCH-07");
}

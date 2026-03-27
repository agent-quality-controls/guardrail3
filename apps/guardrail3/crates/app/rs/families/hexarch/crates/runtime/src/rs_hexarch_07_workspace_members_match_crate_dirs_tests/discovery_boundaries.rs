use super::{copy_fixture, dir_entry, project_tree, write_file};
use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_07_workspace_members_match_crate_dirs as assertions;

#[test]
fn nested_cargo_project_inside_real_leaf_is_still_a_required_workspace_member() {
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

    let results = super::run_family(tmp.path());
    assertions::assert_expected_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            file: Some("apps/devctl/crates/domain/types/examples/demo/Cargo.toml"),
            file_contains: None,
            title_contains: Some(&["crates/domain/types/examples/demo"]),
            message_contains: Some(&["Every live app-local Cargo root must be owned"]),
        }],
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

    let results = super::run_family(tmp.path());
    assertions::assert_no_error(&results, "");
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

    let results = super::run_family(tmp.path());
    assertions::assert_no_error(&results, "");
}

#[test]
fn malformed_nested_cargo_root_without_workspace_member_is_owned_by_rule_07() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/devctl/crates/domain/events/Cargo.toml",
        "[package",
    );

    let results = super::run_family(tmp.path());
    assertions::assert_expected_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            file: Some("apps/devctl/crates/domain/events/Cargo.toml"),
            file_contains: None,
            title_contains: Some(&["crates/domain/events"]),
            message_contains: Some(&["malformed"]),
        }],
    );
    assertions::assert_no_error(&results, "RS-HEXARCH-27");
}

#[test]
fn unreadable_nested_cargo_root_without_workspace_member_fails_closed_under_rule_07() {
    let tree = project_tree(
        vec![
            ("", dir_entry(&["apps"], &[])),
            ("apps", dir_entry(&["devctl"], &[])),
            ("apps/devctl", dir_entry(&["crates"], &["Cargo.toml"])),
            ("apps/devctl/crates", dir_entry(&["domain"], &[])),
            ("apps/devctl/crates/domain", dir_entry(&["types"], &[])),
            (
                "apps/devctl/crates/domain/types",
                dir_entry(&["examples"], &[]),
            ),
            (
                "apps/devctl/crates/domain/types/examples",
                dir_entry(&["demo"], &[]),
            ),
            (
                "apps/devctl/crates/domain/types/examples/demo",
                dir_entry(&[], &["Cargo.toml"]),
            ),
        ],
        vec![(
            "apps/devctl/Cargo.toml",
            "[workspace]\nmembers = [\"crates/domain/types\"]\nresolver = \"2\"\n",
        )],
    );

    let results =
        crate::rs_hexarch_07_workspace_members_match_crate_dirs::results_for_test_tree(&tree);
    assertions::assert_expected_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            file: Some("apps/devctl/crates/domain/types/examples/demo/Cargo.toml"),
            file_contains: None,
            title_contains: Some(&["crates/domain/types/examples/demo"]),
            message_contains: Some(&["Failed to read live app-local Cargo.toml"]),
        }],
    );
}

#[test]
fn excluded_target_cargo_roots_under_app_boundary_do_not_enter_rule_07() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/devctl/target/debug/demo/Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n",
    );
    write_file(
        tmp.path(),
        "apps/devctl/target/debug/demo/src/lib.rs",
        "// excluded demo",
    );

    let results = super::run_family(tmp.path());
    assertions::assert_no_error(&results, "");
}

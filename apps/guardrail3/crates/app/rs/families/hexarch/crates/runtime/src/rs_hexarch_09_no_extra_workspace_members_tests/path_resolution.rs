use super::{copy_fixture, write_file};
use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_09_no_extra_workspace_members as assertions;

#[test]
fn normalized_and_glob_internal_members_are_not_extra_members() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/devctl/Cargo.toml",
        r#"[workspace]
members = [
    "./crates/domain/types/",
    "crates/app/core",
    "crates/ports/outbound/traits",
    "crates/adapters/inbound/cli",
    "crates/adapters/outbound/fs",
]
resolver = "2"
"#,
    );

    let results = super::run_family(tmp.path());
    assertions::assert_no_error(&results, "");
}

#[test]
fn crates_root_member_is_extra_not_outside_boundary() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/devctl/Cargo.toml",
        r#"[workspace]
members = [
    "crates/domain/types",
    "crates/app/core",
    "crates/ports/outbound/traits",
    "crates/adapters/inbound/cli",
    "crates/adapters/outbound/fs",
    "./crates/",
]
resolver = "2"
"#,
    );

    let results = super::run_family(tmp.path());
    assertions::assert_no_error(&results, "RS-HEXARCH-10");
    assertions::assert_expected_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            file: Some("apps/devctl"),
            file_contains: None,
            title_contains: Some(&["./crates/"]),
            message_contains: None,
        }],
    );
}

#[test]
fn leave_and_reenter_same_app_is_not_extra_member() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/devctl/Cargo.toml",
        r#"[workspace]
members = [
    "./../devctl/crates/domain/types/",
    "crates/app/core",
    "crates/ports/outbound/traits",
    "crates/adapters/inbound/cli",
    "crates/adapters/outbound/fs",
]
resolver = "2"
"#,
    );

    let results = super::run_family(tmp.path());
    assertions::assert_no_error(&results, "");
}

#[test]
fn broad_glob_matching_container_dirs_is_still_an_extra_member() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/devctl/Cargo.toml",
        r#"[workspace]
members = [
    "crates/*/*",
    "crates/ports/outbound/traits",
    "crates/adapters/inbound/cli",
    "crates/adapters/outbound/fs",
]
resolver = "2"
"#,
    );

    let results = super::run_family(tmp.path());
    assertions::assert_no_error(&results, "RS-HEXARCH-10");
    assertions::assert_expected_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            file: Some("apps/devctl"),
            file_contains: None,
            title_contains: Some(&["crates/*/*"]),
            message_contains: None,
        }],
    );
}

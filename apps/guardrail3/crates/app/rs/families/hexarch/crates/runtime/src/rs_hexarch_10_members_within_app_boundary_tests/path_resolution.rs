use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_10_members_within_app_boundary as assertions;
use super::{copy_fixture, write_file};

#[test]
fn normalized_internal_members_do_not_trigger_rule_10() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/devctl/Cargo.toml",
        r#"[workspace]
members = [
    "./crates/domain/types/",
    "crates/app/core",
    "crates/ports/outbound/*",
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
fn normalized_outside_boundary_member_still_hits_rule_10() {
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
    "./../../packages/shared-types/",
]
resolver = "2"
"#,
    );

    let results = super::run_family(tmp.path());
    assertions::assert_expected_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            file: Some("apps/devctl"),
            file_contains: None,
            title_contains: Some(&["./../../packages/shared-types/"]),
            message_contains: None,
        }],
    );
}

#[test]
fn parent_escape_into_top_level_crates_still_hits_rule_10() {
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
    "../../packages/shared-types",
]
resolver = "2"
"#,
    );

    let results = super::run_family(tmp.path());
    assertions::assert_expected_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            file: Some("apps/devctl"),
            file_contains: None,
            title_contains: Some(&["../../packages/shared-types"]),
            message_contains: None,
        }],
    );
}

#[test]
fn parent_escape_glob_still_hits_rule_10() {
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
    "../crates/*",
]
resolver = "2"
"#,
    );

    let results_2 = super::run_family(tmp.path());
    assertions::assert_expected_rule_results(
        &results_2,
        &[assertions::ExpectedRuleResult {
            file: Some("apps/devctl"),
            file_contains: None,
            title_contains: Some(&["../crates/*"]),
            message_contains: None,
        }],
    );
}

#[test]
fn leave_and_reenter_same_app_does_not_false_positive_under_rule_10() {
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
fn absolute_member_path_stays_owned_by_rule_10() {
    let tmp = copy_fixture();
    let absolute_member = tmp
        .path()
        .join("apps/devctl/crates/domain/types")
        .display()
        .to_string();
    write_file(
        tmp.path(),
        "apps/devctl/Cargo.toml",
        &format!(
            r#"[workspace]
members = [
    "crates/domain/types",
    "crates/app/core",
    "crates/ports/outbound/traits",
    "crates/adapters/inbound/cli",
    "crates/adapters/outbound/fs",
    "{absolute_member}",
]
resolver = "2"
"#
        ),
    );

    let results = super::run_family(tmp.path());
    assertions::assert_no_error(&results, "RS-HEXARCH-07");
    assertions::assert_no_error(&results, "RS-HEXARCH-09");
    assertions::assert_expected_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            file: Some("apps/devctl"),
            file_contains: None,
            title_contains: Some(&[&absolute_member]),
            message_contains: None,
        }],
    );
}

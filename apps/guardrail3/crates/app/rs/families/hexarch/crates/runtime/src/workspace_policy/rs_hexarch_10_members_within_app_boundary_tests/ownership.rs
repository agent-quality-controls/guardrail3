use super::{copy_fixture, write_file};
use guardrail3_app_rs_family_hexarch_assertions::workspace_policy::rs_hexarch_10_members_within_app_boundary as assertions;

#[test]
fn package_style_app_cargo_is_owned_by_rule_08_not_rule_10() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/devctl/Cargo.toml",
        "[package]\nname = \"devctl\"\nversion = \"0.1.0\"\n",
    );

    let results = super::run_family(tmp.path());
    assertions::assert_no_error(&results, "");
    assertions::assert_error_count(&results, "RS-HEXARCH-08", 1);
}

#[test]
fn phantom_workspace_member_does_not_hit_rule_10() {
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
    "crates/domain/phantom",
]
resolver = "2"
"#,
    );

    let results = super::run_family(tmp.path());
    assertions::assert_no_error(&results, "");
}

#[test]
fn crates_root_member_does_not_hit_rule_10() {
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
    assertions::assert_no_error(&results, "");
}

#[test]
fn app_root_members_are_owned_by_rule_10() {
    for member in [".", "./", ""] {
        let tmp = copy_fixture();
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
    "{member}",
]
resolver = "2"
"#
            ),
        );

        let results = super::run_family(tmp.path());
        let expected = if member.is_empty() {
            assertions::ExpectedRuleResult {
                file: Some("apps/devctl"),
                file_contains: None,
                title_contains: None,
                message_contains: None,
            }
        } else {
            assertions::ExpectedRuleResult {
                file: Some("apps/devctl"),
                file_contains: None,
                title_contains: Some(&[member]),
                message_contains: None,
            }
        };
        assertions::assert_expected_rule_results(&results, &[expected]);
    }
}

#[test]
fn absolute_workspace_member_stays_owned_by_rule_10_not_rule_08() {
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
    assertions::assert_no_error(&results, "RS-HEXARCH-08");
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

#[test]
fn sibling_app_member_stays_owned_by_rule_10() {
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
    "../backend/crates/domain/engine",
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
            title_contains: Some(&["../backend/crates/domain/engine"]),
            message_contains: None,
        }],
    );
}

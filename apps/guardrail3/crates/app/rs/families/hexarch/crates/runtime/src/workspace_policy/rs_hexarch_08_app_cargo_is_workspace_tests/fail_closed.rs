use super::{copy_fixture, write_file};
use guardrail3_app_rs_family_hexarch_assertions::workspace_policy::rs_hexarch_08_app_cargo_is_workspace as assertions;

#[test]
fn parse_error_hits_every_mutated_app() {
    let tmp = copy_fixture();
    for app in ["devctl", "backend", "worker"] {
        write_file(tmp.path(), &format!("apps/{app}/Cargo.toml"), "[workspace");
    }

    let results = super::run_family(tmp.path());
    assertions::assert_expected_rule_results(
        &results,
        &[
            assertions::ExpectedRuleResult {
                file: Some("apps/devctl/Cargo.toml"),
                file_contains: None,
                title_contains: Some(&["invalid workspace config"]),
                message_contains: None,
            },
            assertions::ExpectedRuleResult {
                file: Some("apps/backend/Cargo.toml"),
                file_contains: None,
                title_contains: Some(&["invalid workspace config"]),
                message_contains: None,
            },
            assertions::ExpectedRuleResult {
                file: Some("apps/worker/Cargo.toml"),
                file_contains: None,
                title_contains: Some(&["invalid workspace config"]),
                message_contains: None,
            },
        ],
    );
}

#[test]
fn non_string_workspace_member_is_invalid_app_cargo() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/devctl/Cargo.toml",
        r#"[workspace]
members = [
    "crates/domain/types",
    42,
]
resolver = "2"
"#,
    );

    let results = super::run_family(tmp.path());
    assertions::assert_expected_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            file: Some("apps/devctl/Cargo.toml"),
            file_contains: None,
            title_contains: Some(&["invalid workspace config"]),
            message_contains: Some(&["[workspace].members[1] must be a string"]),
        }],
    );
}

use super::{copy_fixture, write_file};
use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_27_nested_workspace_forbidden as assertions;
use std::path::PathBuf;

#[test]
fn malformed_nested_cargo_is_owned_by_rule_07_not_rule_27() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/devctl/crates/app/rs/families/deny/Cargo.toml",
        "[workspace",
    );

    let results = super::run_family(tmp.path());
    assertions::assert_no_error(&results);
    assertions::assert_workspace_members_error_summary(
        &results,
        1,
        "apps/devctl/crates/app/rs/families/deny/Cargo.toml",
        &["crates/app/rs/families/deny"],
        &["Every live app-local Cargo root must be owned"],
    );
}

#[test]
fn family_style_nested_workspace_shape_hits_rule_27_and_workspace_membership_gap() {
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
    "crates/app/rs/families/deny/crates/runtime",
]
resolver = "2"
"#,
    );
    write_file(
        tmp.path(),
        "apps/devctl/crates/app/rs/families/deny/Cargo.toml",
        "[workspace]\nmembers = [\"crates/runtime\"]\nresolver = \"2\"\n",
    );
    write_file(
        tmp.path(),
        "apps/devctl/crates/app/rs/families/deny/crates/runtime/Cargo.toml",
        "[package]\nname = \"devctl-rs-family-deny-runtime\"\nversion = \"0.1.0\"\n",
    );
    write_file(
        tmp.path(),
        "apps/devctl/crates/app/rs/families/deny/crates/runtime/src/lib.rs",
        "// deny runtime",
    );

    let results = super::run_family(tmp.path());
    assertions::assert_expected_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            file: Some("apps/devctl/crates/app/rs/families/deny/Cargo.toml"),
            file_contains: None,
            title_contains: Some(&["crates/app/rs/families/deny"]),
            message_contains: None,
        }],
    );
    assertions::assert_workspace_members_error_summary(
        &results,
        1,
        "apps/devctl/crates/app/rs/families/deny/Cargo.toml",
        &["crates/app/rs/families/deny"],
        &["Every live app-local Cargo root must be owned"],
    );
}

#[test]
fn actual_guardrail3_repo_no_longer_has_nested_family_workspaces() {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../../../../../../../..")
        .canonicalize()
        .expect("resolve repo root");

    let results = super::run_family(&repo_root);
    assertions::assert_no_error(&results);
    assertions::assert_no_workspace_members_error_with_file_prefix(
        &results,
        "apps/guardrail3/crates/app/rs/families/",
    );
}

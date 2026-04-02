use super::{copy_fixture, write_file};
use guardrail3_app_rs_family_hexarch_assertions::dependency_integrity::rs_hexarch_27_nested_workspace_forbidden as assertions;
use std::path::PathBuf;

#[test]
fn malformed_nested_cargo_is_not_owned_by_rule_27() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/devctl/crates/app/rs/families/deny/Cargo.toml",
        "[workspace",
    );

    let results = super::run_family(tmp.path());
    assertions::assert_no_error(&results);
}

#[test]
fn family_style_nested_workspace_shape_hits_rule_27() {
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
}

#[test]
fn actual_guardrail3_repo_no_longer_has_nested_family_workspaces() {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../../../../../../../..")
        .canonicalize()
        .expect("failed to resolve repository root for nested-workspace ownership test");

    let results = super::run_family(&repo_root);
    assertions::assert_no_error(&results);
}

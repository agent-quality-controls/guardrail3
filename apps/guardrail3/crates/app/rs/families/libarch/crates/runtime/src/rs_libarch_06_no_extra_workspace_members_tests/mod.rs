use guardrail3_app_rs_family_libarch_assertions::rs_libarch_06_no_extra_workspace_members as assertions;
use guardrail3_app_rs_family_libarch_assertions::rs_libarch_06_no_extra_workspace_members::{
    ExpectedRuleResult, Severity,
};

use test_support::{temp_repo, write_layered_library, write_util_member};

const ROOT_CARGO: &str = "packages/shared/Cargo.toml";

#[test]
fn stays_quiet_without_extra_workspace_members() {
    let tmp = temp_repo();
    write_layered_library(tmp.path());

    assertions::assert_rule_quiet(&super::run_family_check(tmp.path()));
}

#[test]
fn errors_when_workspace_includes_extra_member() {
    let tmp = temp_repo();
    write_layered_library(tmp.path());
    write_util_member(tmp.path());
    test_support::write_file(
        tmp.path(),
        ROOT_CARGO,
        "[package]\nname = \"shared\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[workspace]\nmembers = [\"crates/api\", \"crates/core\", \"crates/infra\", \"crates/util\"]\n\n[workspace.dependencies]\napi = { path = \"crates/api\" }\ncore = { path = \"crates/core\" }\ninfra = { path = \"crates/infra\" }\n",
    );

    assertions::assert_rule_results(
        &super::run_family_check(tmp.path()),
        &[ExpectedRuleResult {
            severity: Some(Severity::Error),
            file: Some(ROOT_CARGO),
            message_contains: Some("outside the layered boundary"),
            ..Default::default()
        }],
    );
}

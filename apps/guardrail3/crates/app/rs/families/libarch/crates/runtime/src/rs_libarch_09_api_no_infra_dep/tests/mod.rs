use guardrail3_app_rs_family_libarch_assertions::rs_libarch_09_api_no_infra_dep as assertions;
use guardrail3_app_rs_family_libarch_assertions::rs_libarch_09_api_no_infra_dep::{
    ExpectedRuleResult, Severity,
};

use test_support::{temp_repo, write_layered_library};

const API_CARGO: &str = "packages/shared/crates/api/Cargo.toml";

#[test]
fn inventory_when_api_does_not_depend_on_infra() {
    let tmp = temp_repo();
    write_layered_library(tmp.path());

    assertions::assert_rule_results(
        &super::run_family_check(tmp.path()),
        &[ExpectedRuleResult {
            severity: Some(Severity::Info),
            file: Some(API_CARGO),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}

#[test]
fn errors_when_api_depends_on_infra() {
    let tmp = temp_repo();
    write_layered_library(tmp.path());
    test_support::write_file(
        tmp.path(),
        API_CARGO,
        "[package]\nname = \"api\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\ncore = { workspace = true }\ninfra = { workspace = true }\n",
    );

    assertions::assert_rule_results(
        &super::run_family_check(tmp.path()),
        &[ExpectedRuleResult {
            severity: Some(Severity::Error),
            file: Some(API_CARGO),
            message_contains: Some("api -> infra"),
            ..Default::default()
        }],
    );
}

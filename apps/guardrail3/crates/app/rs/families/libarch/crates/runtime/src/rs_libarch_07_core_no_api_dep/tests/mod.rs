use guardrail3_app_rs_family_libarch_assertions::rs_libarch_07_core_no_api_dep as assertions;
use guardrail3_app_rs_family_libarch_assertions::rs_libarch_07_core_no_api_dep::{
    ExpectedRuleResult, Severity,
};

use test_support::{temp_repo, write_layered_library};

mod golden;

const CORE_CARGO: &str = "packages/shared/crates/core/Cargo.toml";

#[test]
fn inventory_when_core_does_not_depend_on_api() {
    let tmp = temp_repo();
    write_layered_library(tmp.path());

    assertions::assert_rule_results(
        &super::run_family_check(tmp.path()),
        &[ExpectedRuleResult {
            severity: Some(Severity::Info),
            file: Some(CORE_CARGO),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}

#[test]
fn errors_when_core_depends_on_api() {
    let tmp = temp_repo();
    write_layered_library(tmp.path());
    test_support::write_file(
        tmp.path(),
        CORE_CARGO,
        "[package]\nname = \"core\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\napi = { workspace = true }\n",
    );

    assertions::assert_rule_results(
        &super::run_family_check(tmp.path()),
        &[ExpectedRuleResult {
            severity: Some(Severity::Error),
            file: Some(CORE_CARGO),
            message_contains: Some("core -> api"),
            ..Default::default()
        }],
    );
}

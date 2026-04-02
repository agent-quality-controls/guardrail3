use guardrail3_app_rs_family_libarch_assertions::rs_libarch_11_root_facade_exports_api as assertions;
use guardrail3_app_rs_family_libarch_assertions::rs_libarch_11_root_facade_exports_api::{
    ExpectedRuleResult, Severity,
};

use test_support::{temp_repo, write_layered_library};

mod golden;

const ROOT_LIB: &str = "packages/shared/src/lib.rs";

#[test]
fn inventory_when_root_facade_exports_api() {
    let tmp = temp_repo();
    write_layered_library(tmp.path());

    assertions::assert_rule_results(
        &super::run_family_check(tmp.path()),
        &[ExpectedRuleResult {
            severity: Some(Severity::Info),
            file: Some(ROOT_LIB),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}

#[test]
fn errors_when_root_facade_exports_core_instead_of_api() {
    let tmp = temp_repo();
    write_layered_library(tmp.path());
    test_support::write_file(tmp.path(), ROOT_LIB, "pub use core::CoreType;\n");

    assertions::assert_rule_results(
        &super::run_family_check(tmp.path()),
        &[ExpectedRuleResult {
            severity: Some(Severity::Error),
            file: Some(ROOT_LIB),
            message_contains: Some("violates root facade export policy"),
            ..Default::default()
        }],
    );
}

use guardrail3_app_rs_family_toolchain_assertions::rs_toolchain_01_exists::{
    ExpectedRuleResult, assert_rule_results,
};
use guardrail3_domain_report::Severity;

use super::{check, test_input};

#[test]
fn inventories_when_toolchain_toml_exists() {
    let input = test_input(
        Some("rust-toolchain.toml"),
        None,
        None,
        None,
        Some("1.85"),
        None,
    );
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Severity::Info,
            inventory: true,
            title: "rust-toolchain.toml exists",
            message: "Found rust-toolchain.toml at workspace root.",
            file: Some("rust-toolchain.toml"),
        }],
    );
}

#[test]
fn errors_when_no_supported_toolchain_file_exists() {
    let input = test_input(None, None, None, None, Some("1.85"), None);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Severity::Error,
            inventory: false,
            title: "rust-toolchain.toml missing",
            message: "Expected rust-toolchain.toml at workspace root.",
            file: Some(""),
        }],
    );
}

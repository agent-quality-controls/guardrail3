use guardrail3_app_rs_family_toolchain_assertions::rs_toolchain_07_illegal_placement::{
    ExpectedRuleResult, Severity, assert_rule_results,
};

use super::{check, test_input};

#[test]
fn errors_on_modern_toolchain_outside_workspace_root() {
    let input = test_input("apps/admin/rust-toolchain.toml", false);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Severity::Error,
            inventory: false,
            title: "toolchain file outside workspace root",
            message: "`rust-toolchain.toml` at `apps/admin/rust-toolchain.toml` is not at a governed workspace root. Toolchain files are only allowed at workspace roots.",
            file: Some("apps/admin/rust-toolchain.toml"),
        }],
    );
}

#[test]
fn errors_on_legacy_toolchain_outside_workspace_root() {
    let input = test_input("rust-toolchain", true);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Severity::Error,
            inventory: false,
            title: "toolchain file outside workspace root",
            message: "`rust-toolchain` at `rust-toolchain` is not at a governed workspace root. Toolchain files are only allowed at workspace roots.",
            file: Some("rust-toolchain"),
        }],
    );
}

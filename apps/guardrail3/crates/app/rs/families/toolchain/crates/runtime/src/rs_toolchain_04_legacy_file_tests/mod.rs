use guardrail3_app_rs_family_toolchain_assertions::rs_toolchain_04_legacy_file::{
    ExpectedRuleResult, assert_rule_results,
};
use guardrail3_domain_report::Severity;

use super::{check, test_input};

#[test]
fn warns_when_only_legacy_toolchain_file_exists() {
    let input = test_input(None, Some("rust-toolchain"), None, None, None, None);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Severity::Warn,
            inventory: false,
            title: "legacy rust-toolchain file present",
            message: "Migrate `rust-toolchain` to `rust-toolchain.toml` so components can be declared explicitly.",
            file: Some("rust-toolchain"),
        }],
    );
}

#[test]
fn warns_when_both_legacy_and_modern_toolchain_files_exist() {
    let input = test_input(
        Some("rust-toolchain.toml"),
        Some("rust-toolchain"),
        None,
        None,
        None,
        None,
    );
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_rule_results(
        &results,
        &[
            ExpectedRuleResult {
                severity: Severity::Warn,
                inventory: false,
                title: "legacy rust-toolchain file present",
                message: "Migrate `rust-toolchain` to `rust-toolchain.toml` so components can be declared explicitly.",
                file: Some("rust-toolchain"),
            },
            ExpectedRuleResult {
                severity: Severity::Warn,
                inventory: false,
                title: "both rust-toolchain files present",
                message: "Remove the legacy `rust-toolchain` file to avoid ambiguity.",
                file: Some("rust-toolchain"),
            },
        ],
    );
}

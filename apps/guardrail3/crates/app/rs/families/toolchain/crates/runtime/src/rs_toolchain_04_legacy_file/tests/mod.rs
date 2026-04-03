mod helpers;
use guardrail3_app_rs_family_toolchain_assertions::rs_toolchain_04_legacy_file::{
    ExpectedRuleResult, Severity, assert_rule_results,
};

use super::{check, test_input, test_input_for_root};

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
fn errors_when_both_legacy_and_modern_toolchain_files_exist() {
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
                severity: Severity::Error,
                inventory: false,
                title: "both rust-toolchain files present",
                message: "Remove the legacy `rust-toolchain` file. rustup prefers it over `rust-toolchain.toml`, so the modern contract is shadowed.",
                file: Some("rust-toolchain"),
            },
        ],
    );
}

#[test]
fn errors_with_local_legacy_path_for_nested_path_fixture() {
    let input = test_input_for_root(
        "packages/lib",
        "packages/lib/Cargo.toml",
        Some("packages/lib/rust-toolchain.toml"),
        Some("packages/lib/rust-toolchain"),
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
                file: Some("packages/lib/rust-toolchain"),
            },
            ExpectedRuleResult {
                severity: Severity::Error,
                inventory: false,
                title: "both rust-toolchain files present",
                message: "Remove the legacy `rust-toolchain` file. rustup prefers it over `rust-toolchain.toml`, so the modern contract is shadowed.",
                file: Some("packages/lib/rust-toolchain"),
            },
        ],
    );
}

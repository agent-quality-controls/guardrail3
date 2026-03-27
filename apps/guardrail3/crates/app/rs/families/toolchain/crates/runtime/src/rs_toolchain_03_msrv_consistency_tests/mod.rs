use guardrail3_app_rs_family_toolchain_assertions::rs_toolchain_03_msrv_consistency::{
    ExpectedRuleResult, assert_rule_results,
};
use guardrail3_domain_report::Severity;

use super::{
    check, test_input, test_input_invalid_cargo_rust_version_type, test_input_missing_cargo,
};

#[test]
fn warns_when_pinned_toolchain_is_older_than_msrv() {
    let parsed = toml::from_str::<toml::Value>(
        "[toolchain]\nchannel = \"1.84.0\"\ncomponents = [\"clippy\", \"rustfmt\"]",
    )
    .expect("valid TOML");
    let input = test_input(
        Some("rust-toolchain.toml"),
        None,
        Some(&parsed),
        None,
        Some("1.85.0"),
        None,
    );
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Severity::Warn,
            inventory: false,
            title: "pinned toolchain is older than MSRV",
            message: "Pinned toolchain `1.84.0` is older than Cargo rust-version `1.85.0`.",
            file: Some("rust-toolchain.toml"),
        }],
    );
}

#[test]
fn inventories_when_pinned_toolchain_satisfies_msrv() {
    let parsed = toml::from_str::<toml::Value>(
        "[toolchain]\nchannel = \"1.85.1\"\ncomponents = [\"clippy\", \"rustfmt\"]",
    )
    .expect("valid TOML");
    let input = test_input(
        Some("rust-toolchain.toml"),
        None,
        Some(&parsed),
        None,
        Some("1.85.0"),
        None,
    );
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Severity::Info,
            inventory: true,
            title: "pinned toolchain satisfies MSRV",
            message: "Pinned toolchain `1.85.1` is compatible with Cargo rust-version `1.85.0`.",
            file: Some("rust-toolchain.toml"),
        }],
    );
}

#[test]
fn inventories_when_msrv_is_missing() {
    let parsed = toml::from_str::<toml::Value>(
        "[toolchain]\nchannel = \"1.85.1\"\ncomponents = [\"clippy\", \"rustfmt\"]",
    )
    .expect("valid TOML");
    let input = test_input(
        Some("rust-toolchain.toml"),
        None,
        Some(&parsed),
        None,
        None,
        None,
    );
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Severity::Info,
            inventory: true,
            title: "Cargo rust-version not declared",
            message: "No `rust-version` found in Cargo.toml, so MSRV consistency cannot be checked.",
            file: Some("Cargo.toml"),
        }],
    );
}

#[test]
fn errors_when_root_cargo_toml_is_malformed() {
    let parsed = toml::from_str::<toml::Value>(
        "[toolchain]\nchannel = \"1.85.1\"\ncomponents = [\"clippy\", \"rustfmt\"]",
    )
    .expect("valid TOML");
    let input = test_input(
        Some("rust-toolchain.toml"),
        None,
        Some(&parsed),
        None,
        None,
        Some("expected `.`, `=`"),
    );
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Severity::Error,
            inventory: false,
            title: "Cargo.toml parse error blocks MSRV check",
            message: "Invalid root Cargo.toml: expected `.`, `=`",
            file: Some("Cargo.toml"),
        }],
    );
}

#[test]
fn errors_when_root_cargo_toml_is_missing() {
    let parsed = toml::from_str::<toml::Value>(
        "[toolchain]\nchannel = \"1.85.1\"\ncomponents = [\"clippy\", \"rustfmt\"]",
    )
    .expect("valid TOML");
    let input = test_input_missing_cargo(
        Some("rust-toolchain.toml"),
        None,
        Some(&parsed),
        None,
        None,
        None,
    );
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Severity::Error,
            inventory: false,
            title: "Cargo.toml missing blocks MSRV check",
            message: "Root Cargo.toml is required to compare pinned toolchain against declared MSRV.",
            file: Some("Cargo.toml"),
        }],
    );
}

#[test]
fn errors_when_cargo_rust_version_is_invalid() {
    let parsed = toml::from_str::<toml::Value>(
        "[toolchain]\nchannel = \"1.85.1\"\ncomponents = [\"clippy\", \"rustfmt\"]",
    )
    .expect("valid TOML");
    let input = test_input(
        Some("rust-toolchain.toml"),
        None,
        Some(&parsed),
        None,
        Some("stable"),
        None,
    );
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Severity::Error,
            inventory: false,
            title: "Cargo rust-version is invalid",
            message: "Cannot compare pinned toolchain against invalid Cargo rust-version `stable`.",
            file: Some("Cargo.toml"),
        }],
    );
}

#[test]
fn errors_when_cargo_rust_version_is_not_a_string() {
    let parsed = toml::from_str::<toml::Value>(
        "[toolchain]\nchannel = \"1.85.1\"\ncomponents = [\"clippy\", \"rustfmt\"]",
    )
    .expect("valid TOML");
    let input = test_input_invalid_cargo_rust_version_type(
        Some("rust-toolchain.toml"),
        None,
        Some(&parsed),
        None,
        None,
    );
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Severity::Error,
            inventory: false,
            title: "Cargo rust-version is invalid",
            message: "`Cargo.toml` `rust-version` must be a string version.",
            file: Some("Cargo.toml"),
        }],
    );
}

#[test]
fn emits_no_result_for_stable_channel() {
    let parsed = toml::from_str::<toml::Value>(
        "[toolchain]\nchannel = \"stable\"\ncomponents = [\"clippy\", \"rustfmt\"]",
    )
    .expect("valid TOML");
    let input = test_input(
        Some("rust-toolchain.toml"),
        None,
        Some(&parsed),
        None,
        Some("1.85.0"),
        None,
    );
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(results.is_empty());
}

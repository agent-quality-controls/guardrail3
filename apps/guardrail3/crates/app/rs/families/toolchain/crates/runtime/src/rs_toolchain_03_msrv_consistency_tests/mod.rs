use guardrail3_app_rs_family_toolchain_assertions::rs_toolchain_03_msrv_consistency::{
    ExpectedRuleResult, Severity, assert_rule_results,
};

use super::{check, test_input, test_input_invalid_cargo_rust_version_type};

fn parse_toolchain_fixture_toml(source: &str) -> toml::Value {
    toml::from_str::<toml::Value>(source)
        .expect("toolchain MSRV consistency test fixture TOML should parse")
}

#[test]
fn warns_when_pinned_toolchain_is_older_than_msrv() {
    let parsed = parse_toolchain_fixture_toml(
        "[toolchain]\nchannel = \"1.84.0\"\ncomponents = [\"clippy\", \"rustfmt\"]",
    );
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
    let parsed = parse_toolchain_fixture_toml(
        "[toolchain]\nchannel = \"1.85.1\"\ncomponents = [\"clippy\", \"rustfmt\"]",
    );
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
fn inventories_when_pinned_toolchain_with_host_suffix_satisfies_msrv() {
    let parsed = parse_toolchain_fixture_toml(
        "[toolchain]\nchannel = \"1.85.1-x86_64-unknown-linux-gnu\"\ncomponents = [\"clippy\", \"rustfmt\"]",
    );
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
            message: "Pinned toolchain `1.85.1-x86_64-unknown-linux-gnu` is compatible with Cargo rust-version `1.85.0`.",
            file: Some("rust-toolchain.toml"),
        }],
    );
}

#[test]
fn inventories_when_msrv_is_missing() {
    let parsed = parse_toolchain_fixture_toml(
        "[toolchain]\nchannel = \"1.85.1\"\ncomponents = [\"clippy\", \"rustfmt\"]",
    );
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
    let parsed = parse_toolchain_fixture_toml(
        "[toolchain]\nchannel = \"1.85.1\"\ncomponents = [\"clippy\", \"rustfmt\"]",
    );
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
fn errors_when_cargo_rust_version_is_invalid() {
    let parsed = parse_toolchain_fixture_toml(
        "[toolchain]\nchannel = \"1.85.1\"\ncomponents = [\"clippy\", \"rustfmt\"]",
    );
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
    let parsed = parse_toolchain_fixture_toml(
        "[toolchain]\nchannel = \"1.85.1\"\ncomponents = [\"clippy\", \"rustfmt\"]",
    );
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
fn emits_no_result_for_beta_like_version_channel() {
    let parsed = parse_toolchain_fixture_toml(
        "[toolchain]\nchannel = \"1.85.1-beta\"\ncomponents = [\"clippy\", \"rustfmt\"]",
    );
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

#[test]
fn emits_no_result_for_nightly_like_version_channel() {
    let parsed = parse_toolchain_fixture_toml(
        "[toolchain]\nchannel = \"1.85.1-nightly-2026-03-01\"\ncomponents = [\"clippy\", \"rustfmt\"]",
    );
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

#[test]
fn emits_no_result_for_nightly_like_version_channel_after_host_triple() {
    let parsed = parse_toolchain_fixture_toml(
        "[toolchain]\nchannel = \"1.85.1-x86_64-unknown-linux-gnu-nightly\"\ncomponents = [\"clippy\", \"rustfmt\"]",
    );
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

#[test]
fn emits_no_result_for_stable_channel() {
    let parsed = parse_toolchain_fixture_toml(
        "[toolchain]\nchannel = \"stable\"\ncomponents = [\"clippy\", \"rustfmt\"]",
    );
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

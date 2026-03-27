use guardrail3_app_rs_family_toolchain_assertions::rs_toolchain_01_exists::{
    ExpectedRuleResult, assert_rule_results,
};
use guardrail3_domain_report::Severity;

use super::{check, run_family_check, test_input, test_tree};

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

#[test]
fn family_reports_legacy_only_as_missing_modern_toolchain() {
    let tree = test_tree(&["rust-toolchain"], &[]);

    let results = run_family_check(&tree);

    assert_eq!(results.len(), 2, "unexpected family results: {results:#?}");
    assert!(results.iter().any(|result| {
        result.id == "RS-TOOLCHAIN-01"
            && result.severity == Severity::Error
            && result.title == "rust-toolchain.toml missing"
            && result.file.as_deref() == Some("")
    }));
    assert!(results.iter().any(|result| {
        result.id == "RS-TOOLCHAIN-04"
            && result.severity == Severity::Warn
            && result.title == "legacy rust-toolchain file present"
            && result.file.as_deref() == Some("rust-toolchain")
    }));
}

#[test]
fn family_reports_malformed_modern_toolchain_and_legacy_ambiguity() {
    let tree = test_tree(
        &["rust-toolchain.toml", "rust-toolchain"],
        &[("rust-toolchain.toml", "toolchain = [")],
    );

    let results = run_family_check(&tree);

    assert_eq!(results.len(), 4, "unexpected family results: {results:#?}");
    assert!(results.iter().any(|result| {
        result.id == "RS-TOOLCHAIN-01"
            && result.inventory
            && result.severity == Severity::Info
            && result.title == "rust-toolchain.toml exists"
            && result.file.as_deref() == Some("rust-toolchain.toml")
    }));
    assert!(results.iter().any(|result| {
        result.id == "RS-TOOLCHAIN-02"
            && result.severity == Severity::Error
            && result.title == "rust-toolchain.toml parse error"
            && result.file.as_deref() == Some("rust-toolchain.toml")
    }));
    assert!(results.iter().any(|result| {
        result.id == "RS-TOOLCHAIN-04"
            && result.severity == Severity::Warn
            && result.title == "legacy rust-toolchain file present"
            && result.file.as_deref() == Some("rust-toolchain")
    }));
    assert!(results.iter().any(|result| {
        result.id == "RS-TOOLCHAIN-04"
            && result.severity == Severity::Warn
            && result.title == "both rust-toolchain files present"
            && result.file.as_deref() == Some("rust-toolchain")
    }));
}

#[test]
fn family_propagates_invalid_root_cargo_rust_version_type() {
    let tree = test_tree(
        &["rust-toolchain.toml", "Cargo.toml"],
        &[
            (
                "rust-toolchain.toml",
                "[toolchain]\nchannel = \"1.85.1\"\ncomponents = [\"clippy\", \"rustfmt\"]",
            ),
            (
                "Cargo.toml",
                "[package]\nname = \"pkg\"\nedition = \"2024\"\nrust-version = 185",
            ),
        ],
    );

    let results = run_family_check(&tree);

    assert!(results.iter().any(|result| {
        result.id == "RS-TOOLCHAIN-03"
            && result.severity == Severity::Error
            && result.title == "Cargo rust-version is invalid"
            && result.message == "`Cargo.toml` `rust-version` must be a string version."
            && result.file.as_deref() == Some("Cargo.toml")
    }));
}

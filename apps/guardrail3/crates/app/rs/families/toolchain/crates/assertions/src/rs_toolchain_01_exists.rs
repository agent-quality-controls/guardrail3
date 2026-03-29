use guardrail3_domain_report::CheckResult;
pub use guardrail3_domain_report::Severity;

const RULE_ID: &str = "RS-TOOLCHAIN-01";

#[derive(Debug)]
pub struct ExpectedRuleResult<'a> {
    pub severity: Severity,
    pub inventory: bool,
    pub title: &'a str,
    pub message: &'a str,
    pub file: Option<&'a str>,
}

pub fn assert_rule_results(results: &[CheckResult], expected: &[ExpectedRuleResult<'_>]) {
    let actual = results
        .iter()
        .filter(|result| result.id == RULE_ID)
        .collect::<Vec<_>>();
    assert_eq!(
        actual.len(),
        expected.len(),
        "unexpected {RULE_ID} results: {results:#?}"
    );

    for expected_result in expected {
        let matched = actual.iter().any(|result| {
            result.severity == expected_result.severity
                && result.inventory == expected_result.inventory
                && result.title == expected_result.title
                && result.message == expected_result.message
                && result.file.as_deref() == expected_result.file
        });
        assert!(
            matched,
            "missing expected {RULE_ID} result: {expected_result:#?}\nactual: {actual:#?}"
        );
    }
}

pub fn assert_legacy_only_family_results(results: &[CheckResult]) {
    assert_rule_results(
        results,
        &[ExpectedRuleResult {
            severity: Severity::Error,
            inventory: false,
            title: "rust-toolchain.toml missing",
            message: "Expected rust-toolchain.toml at workspace root.",
            file: Some(""),
        }],
    );
    assert!(
        results.iter().any(|result| {
            result.id == "RS-TOOLCHAIN-04"
                && result.severity == Severity::Warn
                && !result.inventory
                && result.title == "legacy rust-toolchain file present"
                && result.message
                    == "Migrate `rust-toolchain` to `rust-toolchain.toml` so components can be declared explicitly."
                && result.file.as_deref() == Some("rust-toolchain")
        }),
        "missing expected RS-TOOLCHAIN-04 legacy result: {results:#?}"
    );
}

pub fn assert_malformed_modern_and_legacy_results(results: &[CheckResult]) {
    assert_rule_results(
        results,
        &[ExpectedRuleResult {
            severity: Severity::Info,
            inventory: true,
            title: "rust-toolchain.toml exists",
            message: "Found rust-toolchain.toml at workspace root.",
            file: Some("rust-toolchain.toml"),
        }],
    );
    assert!(
        results.iter().any(|result| {
            result.id == "RS-TOOLCHAIN-02"
                && result.severity == Severity::Error
                && !result.inventory
                && result.title == "rust-toolchain.toml parse error"
                && result.message.starts_with("Invalid TOML:")
                && result.file.as_deref() == Some("rust-toolchain.toml")
        }),
        "missing expected RS-TOOLCHAIN-02 parse error: {results:#?}"
    );
    assert!(
        results.iter().any(|result| {
            result.id == "RS-TOOLCHAIN-04"
                && result.severity == Severity::Warn
                && !result.inventory
                && result.title == "legacy rust-toolchain file present"
                && result.message
                    == "Migrate `rust-toolchain` to `rust-toolchain.toml` so components can be declared explicitly."
                && result.file.as_deref() == Some("rust-toolchain")
        }),
        "missing expected RS-TOOLCHAIN-04 legacy result: {results:#?}"
    );
    assert!(
        results.iter().any(|result| {
            result.id == "RS-TOOLCHAIN-04"
                && result.severity == Severity::Warn
                && !result.inventory
                && result.title == "both rust-toolchain files present"
                && result.message == "Remove the legacy `rust-toolchain` file to avoid ambiguity."
                && result.file.as_deref() == Some("rust-toolchain")
        }),
        "missing expected RS-TOOLCHAIN-04 ambiguity result: {results:#?}"
    );
}

pub fn assert_invalid_root_cargo_rust_version_type(results: &[CheckResult]) {
    assert!(
        results.iter().any(|result| {
            result.id == "RS-TOOLCHAIN-03"
                && result.severity == Severity::Error
                && !result.inventory
                && result.title == "Cargo rust-version is invalid"
                && result.message == "`Cargo.toml` `rust-version` must be a string version."
                && result.file.as_deref() == Some("Cargo.toml")
        }),
        "missing expected RS-TOOLCHAIN-03 invalid rust-version error: {results:#?}"
    );
}

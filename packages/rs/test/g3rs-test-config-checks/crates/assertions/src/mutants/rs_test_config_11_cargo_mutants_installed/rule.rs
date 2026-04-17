use guardrail3_check_types::{G3CheckResult, G3Severity};

pub fn assert_missing(results: &[G3CheckResult]) {
    assert!(
        results.iter().any(|result| {
            result.id() == "RS-TEST-CONFIG-11"
                && result.severity() == G3Severity::Error
                && result.title() == "cargo-mutants missing"
                && result.file() == Some("Cargo.toml")
        }),
        "missing cargo-mutants missing result: {results:#?}"
    );
}

pub fn assert_present(results: &[G3CheckResult]) {
    assert!(
        results.iter().any(|result| {
            result.id() == "RS-TEST-CONFIG-11"
                && result.severity() == G3Severity::Info
                && result.title() == "cargo-mutants installed"
                && result.file() == Some("Cargo.toml")
        }),
        "missing cargo-mutants installed result: {results:#?}"
    );
}

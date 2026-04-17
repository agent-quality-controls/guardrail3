use guardrail3_check_types::{G3CheckResult, G3Severity};

pub fn assert_missing(results: &[G3CheckResult]) {
    assert!(
        results.iter().any(|result| {
            result.id() == "RS-TEST-CONFIG-13"
                && result.severity() == G3Severity::Error
                && result.title() == "profile.mutants missing"
                && result.file() == Some("Cargo.toml")
        }),
        "missing profile.mutants missing result: {results:#?}"
    );
}

pub fn assert_present(results: &[G3CheckResult]) {
    assert!(
        results.iter().any(|result| {
            result.id() == "RS-TEST-CONFIG-13"
                && result.severity() == G3Severity::Info
                && result.title() == "profile.mutants configured"
                && result.file() == Some("Cargo.toml")
        }),
        "missing profile.mutants configured result: {results:#?}"
    );
}

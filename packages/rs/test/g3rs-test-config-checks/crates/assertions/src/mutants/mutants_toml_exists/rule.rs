use guardrail3_check_types::{G3CheckResult, G3Severity};

pub fn assert_missing(results: &[G3CheckResult]) {
    assert!(
        results.iter().any(|result| {
            result.id() == "g3rs-test/mutants-toml-exists"
                && result.severity() == G3Severity::Error
                && result.title() == "mutants config missing"
                && result.file() == Some(".cargo/mutants.toml")
        }),
        "missing mutants config missing result: {results:#?}"
    );
}

pub fn assert_present(results: &[G3CheckResult]) {
    assert!(
        results.iter().any(|result| {
            result.id() == "g3rs-test/mutants-toml-exists"
                && result.severity() == G3Severity::Info
                && result.title() == "mutants config exists"
                && result.file() == Some(".cargo/mutants.toml")
        }),
        "missing mutants config exists result: {results:#?}"
    );
}

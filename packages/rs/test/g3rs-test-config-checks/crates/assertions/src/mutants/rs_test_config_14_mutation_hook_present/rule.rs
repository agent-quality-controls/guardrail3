use guardrail3_check_types::{G3CheckResult, G3Severity};

pub fn assert_missing(results: &[G3CheckResult]) {
    assert!(
        results.iter().any(|result| {
            result.id() == "RS-TEST-CONFIG-14"
                && result.severity() == G3Severity::Error
                && result.title() == "mutation hook step missing"
                && result.file() == Some("Cargo.toml")
        }),
        "missing mutation hook missing result: {results:#?}"
    );
}

pub fn assert_present(results: &[G3CheckResult]) {
    assert!(
        results.iter().any(|result| {
            result.id() == "RS-TEST-CONFIG-14"
                && result.severity() == G3Severity::Info
                && result.title() == "mutation hook step present"
                && result.file() == Some(".githooks/pre-commit")
        }),
        "missing mutation hook present result: {results:#?}"
    );
}

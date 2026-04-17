use guardrail3_check_types::{G3CheckResult, G3Severity};

pub fn assert_missing(results: &[G3CheckResult]) {
    assert!(
        results.iter().any(|result| {
            result.id() == "RS-TEST-CONFIG-09"
                && result.severity() == G3Severity::Error
                && result.title() == "nextest config missing"
                && result.file() == Some(".config/nextest.toml")
        }),
        "missing nextest missing result: {results:#?}"
    );
}

pub fn assert_configured(results: &[G3CheckResult]) {
    assert!(
        results.iter().any(|result| {
            result.id() == "RS-TEST-CONFIG-09"
                && result.severity() == G3Severity::Info
                && result.title() == "nextest timeouts configured"
                && result.file() == Some(".config/nextest.toml")
        }),
        "missing nextest configured result: {results:#?}"
    );
}

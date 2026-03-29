use guardrail3_domain_modules::clippy::AVOID_BREAKING_EXPORTED_API;
use guardrail3_domain_report::{CheckResult, Severity};

const ID: &str = "RS-CLIPPY-16";

pub fn assert_explicit_false(results: &[CheckResult], file: &str) {
    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, ID);
    assert!(result.inventory);
    assert_eq!(result.severity, Severity::Info);
    assert_eq!(result.title, "avoid-breaking-exported-api explicitly false");
    assert_eq!(
        result.message,
        "`avoid-breaking-exported-api = false` is set."
    );
    assert_eq!(result.file.as_deref(), Some(file));
}

pub fn assert_missing(results: &[CheckResult], file: &str) {
    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, ID);
    assert_eq!(result.severity, Severity::Warn);
    assert_eq!(result.title, "avoid-breaking-exported-api not set");
    assert_eq!(
        result.message,
        "Set `avoid-breaking-exported-api = false` explicitly unless this is a published library."
    );
    assert!(!result.inventory);
    assert_eq!(result.file.as_deref(), Some(file));
}

pub fn assert_published_library(results: &[CheckResult], file: &str) {
    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, ID);
    assert!(result.inventory);
    assert_eq!(result.severity, Severity::Info);
    assert_eq!(
        result.title,
        "library keeps avoid-breaking-exported-api enabled"
    );
    assert_eq!(
        result.message,
        "Published library profile may legitimately keep `avoid-breaking-exported-api = true`."
    );
    assert_eq!(result.file.as_deref(), Some(file));
}

pub fn assert_warn_true(results: &[CheckResult], file: &str) {
    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, ID);
    assert_eq!(result.severity, Severity::Warn);
    assert_eq!(result.title, "avoid-breaking-exported-api enabled");
    assert_eq!(
        result.message,
        "`avoid-breaking-exported-api = true` suppresses useful lints. Prefer `false`."
    );
    assert!(!result.inventory);
    assert_eq!(result.file.as_deref(), Some(file));
}

pub fn assert_generated_service_baseline_contains_expected_value(clippy_toml: &str) {
    let parsed = toml::from_str::<toml::Value>(clippy_toml).expect("valid clippy TOML");

    assert_eq!(
        parsed
            .get("avoid-breaking-exported-api")
            .and_then(toml::Value::as_bool),
        Some(AVOID_BREAKING_EXPORTED_API)
    );
}

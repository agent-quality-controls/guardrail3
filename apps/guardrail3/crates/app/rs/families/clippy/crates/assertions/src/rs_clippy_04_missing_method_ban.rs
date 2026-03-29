use std::collections::BTreeSet;

use guardrail3_domain_modules::clippy::SERVICE_METHOD_PATHS;
use guardrail3_domain_report::{CheckResult, Severity};

const ID: &str = "RS-CLIPPY-04";

pub fn assert_golden(results: &[CheckResult], file: &str) {
    assert!(!results.is_empty());
    assert!(results.iter().all(|result| {
        result.id == ID
            && result.inventory
            && result.severity == Severity::Info
            && result.title == "method ban present"
            && result.file.as_deref() == Some(file)
    }));
    assert!(
        results
            .iter()
            .any(|result| result.message == "`std::env::var` is banned.")
    );
    assert!(
        results
            .iter()
            .any(|result| result.message == "`std::process::abort` is banned.")
    );
}

pub fn assert_garde_disabled(results: &[CheckResult], file: &str) {
    assert!(results.iter().all(|result| {
        result.id == ID
            && result.inventory
            && result.severity == Severity::Info
            && result.file.as_deref() == Some(file)
    }));
    assert!(
        !results
            .iter()
            .any(|result| result.message.contains("serde_json::from_str"))
    );
    assert!(
        !results
            .iter()
            .any(|result| result.message.contains("reqwest::Response::json"))
    );
}

pub fn assert_missing_messages(results: &[CheckResult], expected: &[&str]) {
    let actual_errors = results
        .iter()
        .filter(|result| result.severity == Severity::Error)
        .map(|result| result.message.clone())
        .collect::<BTreeSet<_>>();
    let expected_errors = expected
        .iter()
        .map(|message| (*message).to_owned())
        .collect::<BTreeSet<_>>();
    assert_eq!(actual_errors, expected_errors);
    assert!(results.iter().all(|result| result.id == ID));
}

pub fn assert_generated_service_method_ban_set(clippy_toml: &str) {
    let parsed = toml::from_str::<toml::Value>(clippy_toml).expect("valid clippy TOML");
    let actual = parsed
        .get("disallowed-methods")
        .and_then(toml::Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|entry| entry.get("path").and_then(toml::Value::as_str))
        .map(str::to_owned)
        .collect::<BTreeSet<_>>();
    let expected = SERVICE_METHOD_PATHS
        .iter()
        .map(|path| (*path).to_owned())
        .collect::<BTreeSet<_>>();

    assert_eq!(actual, expected);
}

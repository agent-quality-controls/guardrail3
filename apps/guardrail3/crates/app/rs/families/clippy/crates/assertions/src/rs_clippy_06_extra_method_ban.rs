use std::collections::BTreeSet;

use guardrail3_domain_modules::clippy::SERVICE_METHOD_PATHS;
use guardrail3_domain_report::{CheckResult, Severity};

const ID: &str = "RS-CLIPPY-06";

pub fn assert_no_results(results: &[CheckResult]) {
    assert!(
        results.is_empty(),
        "expected no extra method-ban findings: {results:#?}"
    );
}

pub fn assert_messages(results: &[CheckResult], expected: &[&str], file: &str) {
    let actual_messages = results
        .iter()
        .map(|result| result.message.clone())
        .collect::<BTreeSet<_>>();
    let expected_messages = expected
        .iter()
        .map(|message| (*message).to_owned())
        .collect::<BTreeSet<_>>();

    assert_eq!(actual_messages, expected_messages);
    assert!(results.iter().all(|result| {
        result.id == ID
            && result.inventory
            && result.severity == Severity::Info
            && result.file.as_deref() == Some(file)
    }));
}

pub fn assert_project_specific(results: &[CheckResult], path: &str, file: &str) {
    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, ID);
    assert!(result.inventory);
    assert_eq!(result.severity, Severity::Info);
    assert_eq!(result.title, "extra method ban");
    assert_eq!(
        result.message,
        format!("Additional method ban `{path}` beyond baseline.")
    );
    assert_eq!(result.file.as_deref(), Some(file));
}

pub fn assert_generated_service_methods_do_not_contain_project_specific_extras(clippy_toml: &str) {
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

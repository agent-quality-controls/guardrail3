use std::collections::BTreeSet;

use guardrail3_domain_modules::clippy::{BASE_TYPE_PATHS, LIBRARY_EXTRA_TYPE_PATHS};
use guardrail3_domain_report::{CheckResult, Severity};

const ID: &str = "RS-CLIPPY-14";

pub fn assert_no_results(results: &[CheckResult]) {
    assert!(
        results.is_empty(),
        "expected no library global-state findings: {results:#?}"
    );
}

pub fn assert_missing_messages(results: &[CheckResult], expected: &[&str], file: &str) {
    let actual_messages = results
        .iter()
        .map(|result| result.message.clone())
        .collect::<BTreeSet<_>>();
    let expected_messages = expected
        .iter()
        .map(|message| (*message).to_owned())
        .collect::<BTreeSet<_>>();

    assert_eq!(actual_messages, expected_messages);
    assert_eq!(results.len(), expected_messages.len());
    assert!(results.iter().all(|result| {
        result.id == ID
            && !result.inventory
            && result.severity == Severity::Error
            && result.title == "library clippy.toml missing global-state type ban"
            && result.file.as_deref() == Some(file)
    }));
}

pub fn assert_generated_library_profile_contains_exact_managed_global_state_type_set(
    clippy_toml: &str,
) {
    let parsed = toml::from_str::<toml::Value>(clippy_toml).expect("valid clippy TOML");
    let actual = parsed
        .get("disallowed-types")
        .and_then(toml::Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|entry| entry.get("path").and_then(toml::Value::as_str))
        .map(str::to_owned)
        .collect::<BTreeSet<_>>();
    let expected = BASE_TYPE_PATHS
        .iter()
        .chain(LIBRARY_EXTRA_TYPE_PATHS.iter())
        .map(|path| (*path).to_owned())
        .collect::<BTreeSet<_>>();

    assert_eq!(actual, expected);
}

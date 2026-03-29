use std::collections::BTreeSet;

use guardrail3_domain_modules::clippy::EXPECTED_MACRO_BANS;
use guardrail3_domain_report::{CheckResult, Severity};

const ID: &str = "RS-CLIPPY-20";

pub fn assert_golden(results: &[CheckResult], expected: &[&str], file: &str) {
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
            && result.title == "macro ban present"
            && result.file.as_deref() == Some(file)
    }));
}

pub fn assert_missing_messages(results: &[CheckResult], expected: &[&str]) {
    let error_messages = results
        .iter()
        .filter(|result| result.severity == Severity::Error)
        .map(|result| result.message.clone())
        .collect::<BTreeSet<_>>();
    let expected_error_messages = expected
        .iter()
        .map(|message| (*message).to_owned())
        .collect::<BTreeSet<_>>();

    assert_eq!(error_messages, expected_error_messages);
    assert!(results.iter().all(|result| result.id == ID));
}

pub fn assert_generated_macro_ban_set_matches_rule_baseline(clippy_toml: &str) {
    let parsed = toml::from_str::<toml::Value>(clippy_toml).expect("valid clippy TOML");
    let actual = parsed
        .get("disallowed-macros")
        .and_then(toml::Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|entry| entry.get("path").and_then(toml::Value::as_str))
        .map(str::to_owned)
        .collect::<BTreeSet<_>>();
    let expected = EXPECTED_MACRO_BANS
        .iter()
        .map(|path| (*path).to_owned())
        .collect::<BTreeSet<_>>();

    assert_eq!(actual, expected);
}

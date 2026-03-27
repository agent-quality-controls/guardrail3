use guardrail3_domain_report::{CheckResult, Severity};
#[cfg(test)]
use guardrail3_domain_project_tree::ProjectTree;

use super::inputs::ConfigClippyInput;

const ID: &str = "RS-CLIPPY-17";

pub fn check(input: &ConfigClippyInput<'_>, results: &mut Vec<CheckResult>) {
    let Some(parsed) = input.config.parsed.as_ref() else {
        return;
    };

    for (key, expected, severity, title, message) in [
        (
            "allow-dbg-in-tests",
            false,
            Severity::Warn,
            "clippy test relaxation enabled",
            "`allow-dbg-in-tests = true` relaxes test output discipline.",
        ),
        (
            "allow-print-in-tests",
            false,
            Severity::Warn,
            "clippy test relaxation enabled",
            "`allow-print-in-tests = true` relaxes test output discipline.",
        ),
        (
            "allow-expect-in-tests",
            true,
            Severity::Error,
            "clippy test expect policy misconfigured",
            "`allow-expect-in-tests` must be `true` so tests may use `expect(...)` while non-test code stays governed by `clippy::expect_used`.",
        ),
        (
            "allow-panic-in-tests",
            false,
            Severity::Error,
            "clippy test panic relaxation enabled",
            "`allow-panic-in-tests` must stay `false` so `panic!()` remains banned in tests.",
        ),
        (
            "allow-unwrap-in-tests",
            false,
            Severity::Error,
            "clippy test unwrap relaxation enabled",
            "`allow-unwrap-in-tests` must stay `false` so `unwrap()` remains banned in tests.",
        ),
    ] {
        if parsed.get(key).and_then(toml::Value::as_bool) != Some(expected) {
            results.push(CheckResult {
                id: ID.to_owned(),
                severity,
                title: title.to_owned(),
                message: message.to_owned(),
                file: Some(input.config.rel_path.clone()),
                line: None,
                inventory: false,
            });
        }
    }
}

#[cfg(test)]
pub(crate) fn run_for_tests(tree: &ProjectTree, rel_path: &str) -> Vec<CheckResult> {
    let facts = super::facts::collect_for_tests(tree);
    let mut results = Vec::new();
    check(&super::facts::config_input_for_tests(&facts, rel_path), &mut results);
    results
}

#[cfg(test)]
#[path = "rs_clippy_17_test_relaxations_tests/mod.rs"]
mod rs_clippy_17_test_relaxations_tests;

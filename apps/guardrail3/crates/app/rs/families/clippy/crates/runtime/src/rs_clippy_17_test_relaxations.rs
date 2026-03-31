#[cfg(test)]
use guardrail3_domain_project_tree::ProjectTree;
use guardrail3_domain_report::{CheckResult, Severity};

use super::clippy_support::{BoolSetting, bool_setting, value_kind};
use super::inputs::ConfigClippyInput;

const ID: &str = "RS-CLIPPY-17";

pub fn check(input: &ConfigClippyInput<'_>, results: &mut Vec<CheckResult>) {
    let Some(parsed) = input.config.parsed.as_ref() else {
        return;
    };

    let mut mismatch_count = 0usize;

    for (key, expected, severity, title) in [
        (
            "allow-dbg-in-tests",
            false,
            Severity::Warn,
            "clippy test relaxation enabled",
        ),
        (
            "allow-print-in-tests",
            false,
            Severity::Warn,
            "clippy test relaxation enabled",
        ),
        (
            "allow-expect-in-tests",
            true,
            Severity::Error,
            "clippy test expect policy misconfigured",
        ),
        (
            "allow-panic-in-tests",
            false,
            Severity::Error,
            "clippy test panic relaxation enabled",
        ),
        (
            "allow-unwrap-in-tests",
            false,
            Severity::Error,
            "clippy test unwrap relaxation enabled",
        ),
    ] {
        match bool_setting(parsed, key) {
            BoolSetting::Value(actual) if actual == expected => {}
            BoolSetting::Value(actual) => {
                mismatch_count += 1;
                results.push(CheckResult {
                    id: ID.to_owned(),
                    severity,
                    title: title.to_owned(),
                    message: relaxation_message(key, expected, Some(actual), None),
                    file: Some(input.config.rel_path.clone()),
                    line: None,
                    inventory: false,
                });
            }
            BoolSetting::WrongType(value) => {
                mismatch_count += 1;
                results.push(CheckResult {
                    id: ID.to_owned(),
                    severity,
                    title: format!("{title} wrong type"),
                    message: relaxation_message(key, expected, None, Some(value_kind(value))),
                    file: Some(input.config.rel_path.clone()),
                    line: None,
                    inventory: false,
                });
            }
            BoolSetting::Missing => {
                mismatch_count += 1;
                results.push(CheckResult {
                    id: ID.to_owned(),
                    severity,
                    title: format!("{title} missing"),
                    message: relaxation_message(key, expected, None, None),
                    file: Some(input.config.rel_path.clone()),
                    line: None,
                    inventory: false,
                });
            }
        }
    }

    if mismatch_count == 0 {
        results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Info,
                title: "clippy test relaxation policy exact".to_owned(),
                message: "Managed test relaxation keys match the expected clippy policy."
                    .to_owned(),
                file: Some(input.config.rel_path.clone()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
    }
}

#[cfg(test)]
pub(crate) fn run_for_tests(tree: &ProjectTree, rel_path: &str) -> Vec<CheckResult> {
    let facts = super::facts::collect_for_tests(tree);
    let mut results = Vec::new();
    check(
        &super::facts::config_input_for_tests(&facts, rel_path),
        &mut results,
    );
    results
}

#[cfg(test)]
#[path = "rs_clippy_17_test_relaxations_tests/mod.rs"] // reason: test-only sidecar module wiring
mod rs_clippy_17_test_relaxations_tests;

fn relaxation_message(
    key: &str,
    expected: bool,
    actual: Option<bool>,
    wrong_type: Option<&str>,
) -> String {
    let policy = match key {
        "allow-dbg-in-tests" | "allow-print-in-tests" => {
            "Tests should stay quiet and deterministic."
        }
        "allow-expect-in-tests" => {
            "Tests may use `expect(...)` while non-test code stays governed by `clippy::expect_used`."
        }
        "allow-panic-in-tests" => "panic!() must remain banned in tests.",
        "allow-unwrap-in-tests" => "unwrap() must remain banned in tests.",
        _ => "Managed test relaxation keys must match the hardened clippy policy.",
    };

    match (actual, wrong_type) {
        (Some(actual), None) => format!("`{key}` must be `{expected}`; found `{actual}`. {policy}"),
        (None, Some(kind)) => {
            format!("`{key}` must be a bool with value `{expected}`, found {kind}. {policy}")
        }
        (None, None) => format!("`{key}` must be set explicitly to `{expected}`. {policy}"),
        (Some(_), Some(_)) => unreachable!("actual bool and wrong-type marker are exclusive"),
    }
}

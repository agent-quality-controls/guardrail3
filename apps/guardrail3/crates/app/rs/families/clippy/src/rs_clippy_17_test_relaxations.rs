use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::ConfigClippyInput;

const ID: &str = "RS-CLIPPY-17";

pub fn check(input: &ConfigClippyInput<'_>, results: &mut Vec<CheckResult>) {
    let Some(parsed) = input.config.parsed.as_ref() else {
        return;
    };

    for key in ["allow-dbg-in-tests", "allow-print-in-tests"] {
        if parsed.get(key).and_then(toml::Value::as_bool) == Some(true) {
            results.push(CheckResult {
                id: ID.to_owned(),
                severity: Severity::Warn,
                title: "clippy test relaxation enabled".to_owned(),
                message: format!("`{key} = true` relaxes test output discipline."),
                file: Some(input.config.rel_path.clone()),
                line: None,
                inventory: false,
            });
        }
    }
}

#[cfg(test)]
#[path = "rs_clippy_17_test_relaxations_tests/mod.rs"]
mod tests;

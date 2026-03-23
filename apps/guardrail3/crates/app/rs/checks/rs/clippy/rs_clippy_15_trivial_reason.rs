use crate::domain::report::{CheckResult, Severity};

use super::clippy_support::{is_placeholder_reason, parse_ban_entries};
use super::inputs::ConfigClippyInput;

const ID: &str = "RS-CLIPPY-15";

pub fn check(input: &ConfigClippyInput<'_>, results: &mut Vec<CheckResult>) {
    let Some(parsed) = input.config.parsed.as_ref() else {
        return;
    };

    for key in [
        "disallowed-methods",
        "disallowed-types",
        "disallowed-macros",
    ] {
        for entry in parse_ban_entries(parsed, key) {
            if let Some(reason) = entry.reason.as_deref()
                && is_placeholder_reason(reason)
            {
                results.push(CheckResult {
                    id: ID.to_owned(),
                    severity: Severity::Warn,
                    title: "ban entry has placeholder reason".to_owned(),
                    message: format!(
                        "`{}` in `{key}` has a trivial or placeholder `reason`.",
                        entry.path
                    ),
                    file: Some(input.config.rel_path.clone()),
                    line: None,
                    inventory: false,
                });
            }
        }
    }
}

#[cfg(test)]
#[path = "rs_clippy_15_trivial_reason_tests/mod.rs"]
mod tests;

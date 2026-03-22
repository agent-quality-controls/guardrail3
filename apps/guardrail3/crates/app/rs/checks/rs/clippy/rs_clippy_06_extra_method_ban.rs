use std::collections::BTreeSet;

use crate::domain::report::{CheckResult, Severity};

use super::clippy_support::{EXPECTED_METHOD_BANS, ban_paths};
use super::inputs::ConfigClippyInput;

const ID: &str = "RS-CLIPPY-06";

pub fn check(input: &ConfigClippyInput<'_>, results: &mut Vec<CheckResult>) {
    let Some(parsed) = input.config.parsed.as_ref() else {
        return;
    };

    let expected: BTreeSet<_> = EXPECTED_METHOD_BANS.iter().copied().collect();
    for found in ban_paths(parsed, "disallowed-methods") {
        if !expected.contains(found.as_str()) {
            results.push(
                CheckResult {
                    id: ID.to_owned(),
                    severity: Severity::Info,
                    title: "extra method ban".to_owned(),
                    message: format!("Additional method ban `{found}` beyond baseline."),
                    file: Some(input.config.rel_path.clone()),
                    line: None,
                    inventory: false,
                }
                .as_inventory(),
            );
        }
    }
}

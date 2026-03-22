use std::collections::BTreeSet;

use crate::domain::report::{CheckResult, Severity};

use super::clippy_support::ban_paths;
use super::inputs::ConfigClippyInput;

const ID: &str = "RS-CLIPPY-04";

pub fn check(input: &ConfigClippyInput<'_>, results: &mut Vec<CheckResult>) {
    let Some(parsed) = input.config.parsed.as_ref() else {
        return;
    };

    let found: BTreeSet<_> = ban_paths(parsed, "disallowed-methods")
        .into_iter()
        .collect();
    for expected in super::clippy_support::expected_method_bans(input.garde_enabled()) {
        if found.contains(expected) {
            results.push(
                CheckResult {
                    id: ID.to_owned(),
                    severity: Severity::Info,
                    title: "method ban present".to_owned(),
                    message: format!("`{expected}` is banned."),
                    file: Some(input.config.rel_path.clone()),
                    line: None,
                    inventory: false,
                }
                .as_inventory(),
            );
        } else {
            results.push(CheckResult {
                id: ID.to_owned(),
                severity: Severity::Error,
                title: "missing method ban".to_owned(),
                message: format!("`{expected}` is not present in `disallowed-methods`."),
                file: Some(input.config.rel_path.clone()),
                line: None,
                inventory: false,
            });
        }
    }
}

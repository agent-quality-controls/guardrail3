use std::collections::BTreeSet;

use crate::domain::report::{CheckResult, Severity};

use super::clippy_support::{ban_paths, expected_type_bans};
use super::inputs::ConfigClippyInput;

const ID: &str = "RS-CLIPPY-05";

pub fn check(input: &ConfigClippyInput<'_>, results: &mut Vec<CheckResult>) {
    let Some(parsed) = input.config.parsed.as_ref() else {
        return;
    };

    let found: BTreeSet<_> = ban_paths(parsed, "disallowed-types").into_iter().collect();
    for expected in expected_type_bans(input.profile_name(), input.garde_enabled()) {
        if found.contains(expected) {
            results.push(
                CheckResult {
                    id: ID.to_owned(),
                    severity: Severity::Info,
                    title: "type ban present".to_owned(),
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
                title: "missing type ban".to_owned(),
                message: format!("`{expected}` is not present in `disallowed-types`."),
                file: Some(input.config.rel_path.clone()),
                line: None,
                inventory: false,
            });
        }
    }
}

#[cfg(test)]
#[path = "rs_clippy_05_missing_type_ban_tests.rs"]
mod tests;

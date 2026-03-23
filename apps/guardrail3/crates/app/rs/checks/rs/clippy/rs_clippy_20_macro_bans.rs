use std::collections::BTreeSet;

use crate::domain::report::{CheckResult, Severity};

use super::clippy_support::{EXPECTED_MACRO_BANS, ban_paths};
use super::inputs::ConfigClippyInput;

const ID: &str = "RS-CLIPPY-20";

pub fn check(input: &ConfigClippyInput<'_>, results: &mut Vec<CheckResult>) {
    let Some(parsed) = input.config.parsed.as_ref() else {
        return;
    };

    let found: BTreeSet<_> = ban_paths(parsed, "disallowed-macros").into_iter().collect();
    for expected in EXPECTED_MACRO_BANS {
        if found.contains(*expected) {
            results.push(
                CheckResult {
                    id: ID.to_owned(),
                    severity: Severity::Info,
                    title: "macro ban present".to_owned(),
                    message: format!("`{expected}!` is banned."),
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
                title: "missing macro ban".to_owned(),
                message: format!("`{expected}!` is not present in `disallowed-macros`."),
                file: Some(input.config.rel_path.clone()),
                line: None,
                inventory: false,
            });
        }
    }
}

#[cfg(test)]
#[path = "rs_clippy_20_macro_bans_tests/mod.rs"]
mod tests;

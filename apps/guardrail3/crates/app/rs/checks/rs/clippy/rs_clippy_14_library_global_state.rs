use std::collections::BTreeSet;

use crate::domain::report::{CheckResult, Severity};

use super::clippy_support::{EXPECTED_LIBRARY_GLOBAL_STATE_TYPES, ban_paths};
use super::inputs::ConfigClippyInput;

const ID: &str = "RS-CLIPPY-14";

pub fn check(input: &ConfigClippyInput<'_>, results: &mut Vec<CheckResult>) {
    if input.profile_name() != Some("library") {
        return;
    }
    let Some(parsed) = input.config.parsed.as_ref() else {
        return;
    };

    let found: BTreeSet<_> = ban_paths(parsed, "disallowed-types").into_iter().collect();
    for expected in EXPECTED_LIBRARY_GLOBAL_STATE_TYPES {
        if !found.contains(*expected) {
            results.push(CheckResult {
                id: ID.to_owned(),
                severity: Severity::Error,
                title: "library clippy.toml missing global-state type ban".to_owned(),
                message: format!("Library profile must ban `{expected}` in `disallowed-types`."),
                file: Some(input.config.rel_path.clone()),
                line: None,
                inventory: false,
            });
        }
    }
}

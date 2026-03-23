use crate::domain::report::{CheckResult, Severity};

use super::inputs::ExecutableCommandContextInput;

const ID: &str = "HOOK-SHARED-10";

pub fn check(input: &ExecutableCommandContextInput<'_>, results: &mut Vec<CheckResult>) {
    let has_shell_error_handling = input.content.lines().any(|line| {
        let trimmed = line.trim();
        trimmed == "set -e"
            || trimmed == "set -eu"
            || trimmed == "set -eo pipefail"
            || trimmed == "set -euo pipefail"
            || trimmed.contains("set -e")
    });

    if has_shell_error_handling {
        results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Warn,
                title: "shell error handling present".to_owned(),
                message: "Hook enables shell error handling with `set -e` or equivalent."
                    .to_owned(),
                file: Some(input.rel_path.to_owned()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
    } else {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Warn,
            title: "shell error handling missing".to_owned(),
            message: "Hook does not enable `set -e`-style shell error handling.".to_owned(),
            file: Some(input.rel_path.to_owned()),
            line: None,
            inventory: false,
        });
    }
}

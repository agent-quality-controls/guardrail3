use crate::domain::report::{CheckResult, Severity};

use super::inputs::RustHookCommandInput;

const ID: &str = "HOOK-RS-09";

pub fn check(input: &RustHookCommandInput<'_>, results: &mut Vec<CheckResult>) {
    let found = input.parsed.executable_lines.iter().any(|line| {
        line.command_name == "cargo"
            && line.command_text.contains("cargo clippy")
            && (line.command_text.contains("-D warnings")
                || line.command_text.contains("--deny warnings"))
    });

    if found {
        results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Warn,
                title: "cargo clippy denies warnings".to_owned(),
                message: "Hook runs clippy in a deny-warnings mode.".to_owned(),
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
            title: "cargo clippy deny-warnings step missing".to_owned(),
            message: "Hook does not execute `cargo clippy` with `-D warnings` or equivalent."
                .to_owned(),
            file: Some(input.rel_path.to_owned()),
            line: None,
            inventory: false,
        });
    }
}

#[cfg(test)]
#[path = "hook_rs_09_clippy_denies_warnings_tests.rs"]
mod tests;

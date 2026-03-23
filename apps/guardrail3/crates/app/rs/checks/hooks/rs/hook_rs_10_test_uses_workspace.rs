use crate::domain::report::{CheckResult, Severity};

use super::inputs::RustHookCommandInput;

const ID: &str = "HOOK-RS-10";

pub fn check(input: &RustHookCommandInput<'_>, results: &mut Vec<CheckResult>) {
    let found = input.parsed.executable_lines.iter().any(|line| {
        line.command_name == "cargo"
            && line.command_text.contains("cargo test")
            && line.command_text.contains("--workspace")
    });

    if found {
        results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Info,
                title: "cargo test uses workspace scope".to_owned(),
                message: "Hook runs cargo test with `--workspace`.".to_owned(),
                file: Some(input.rel_path.to_owned()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
    } else {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Info,
            title: "cargo test workspace scope missing".to_owned(),
            message: "Hook does not execute `cargo test --workspace`.".to_owned(),
            file: Some(input.rel_path.to_owned()),
            line: None,
            inventory: false,
        });
    }
}

#[cfg(test)]
#[path = "hook_rs_10_test_uses_workspace_tests.rs"]
mod tests;

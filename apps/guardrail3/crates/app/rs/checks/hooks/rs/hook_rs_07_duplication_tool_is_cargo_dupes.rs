use crate::domain::report::{CheckResult, Severity};

use super::inputs::RustHookCommandInput;

const ID: &str = "HOOK-RS-07";

pub fn check(input: &RustHookCommandInput<'_>, results: &mut Vec<CheckResult>) {
    let has_cargo_dupes = input.parsed.executable_lines.iter().any(|line| {
        (line.command_name == "cargo" && line.command_text.contains("cargo dupes"))
            || line.command_name == "cargo-dupes"
    });
    let has_jscpd = input
        .parsed
        .executable_lines
        .iter()
        .any(|line| line.command_name == "jscpd");

    if has_jscpd && !has_cargo_dupes {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Warn,
            title: "wrong Rust duplication tool".to_owned(),
            message: "Hook uses jscpd for Rust duplication checks instead of cargo-dupes."
                .to_owned(),
            file: Some(input.rel_path.to_owned()),
            line: None,
            inventory: false,
        });
    } else if has_cargo_dupes {
        results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Warn,
                title: "cargo-dupes selected for Rust duplication checks".to_owned(),
                message: "Hook uses cargo-dupes for Rust duplication checks.".to_owned(),
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
            title: "Rust duplication tool missing".to_owned(),
            message: "Hook does not show a Rust duplication-check command.".to_owned(),
            file: Some(input.rel_path.to_owned()),
            line: None,
            inventory: false,
        });
    }
}

#[cfg(test)]
#[path = "hook_rs_07_duplication_tool_is_cargo_dupes_tests.rs"]
mod tests;

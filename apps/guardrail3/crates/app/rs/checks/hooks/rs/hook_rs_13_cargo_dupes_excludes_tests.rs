use crate::domain::report::{CheckResult, Severity};

use super::inputs::RustHookCommandInput;

const ID: &str = "HOOK-RS-13";

pub fn check(input: &RustHookCommandInput<'_>, results: &mut Vec<CheckResult>) {
    let found = input.parsed.executable_lines.iter().any(|line| {
        ((line.command_name == "cargo" && line.command_text.contains("cargo dupes"))
            || line.command_name == "cargo-dupes")
            && line.command_text.contains("--exclude-tests")
    });

    if found {
        results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Info,
                title: "cargo-dupes excludes tests".to_owned(),
                message: "Hook runs cargo-dupes with `--exclude-tests`.".to_owned(),
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
            title: "cargo-dupes exclude-tests flag missing".to_owned(),
            message: "Hook does not execute cargo-dupes with `--exclude-tests`.".to_owned(),
            file: Some(input.rel_path.to_owned()),
            line: None,
            inventory: false,
        });
    }
}

#[cfg(test)]
#[path = "hook_rs_13_cargo_dupes_excludes_tests_tests.rs"]
mod tests;

use crate::domain::report::{CheckResult, Severity};
use crate::ports::outbound::ToolChecker;

const ID: &str = "HOOK-RS-15";

pub fn check(rel_path: &str, tc: &dyn ToolChecker, results: &mut Vec<CheckResult>) {
    if tc.is_installed("cargo-dupes") {
        results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Error,
                title: "cargo-dupes installed".to_owned(),
                message: "cargo-dupes is available for Rust duplication checks.".to_owned(),
                file: Some(rel_path.to_owned()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
    } else {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: "cargo-dupes missing".to_owned(),
            message: "Hook requires cargo-dupes, but it is not available on PATH.".to_owned(),
            file: Some(rel_path.to_owned()),
            line: None,
            inventory: false,
        });
    }
}

#[cfg(test)]
#[path = "hook_rs_15_cargo_dupes_installed_tests.rs"]
mod tests;

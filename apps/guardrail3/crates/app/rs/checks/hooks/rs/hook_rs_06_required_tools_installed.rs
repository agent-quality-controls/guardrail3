use crate::domain::report::{CheckResult, Severity};
use crate::ports::outbound::ToolChecker;

const ID: &str = "HOOK-RS-06";

pub fn check(rel_path: &str, tc: &dyn ToolChecker, results: &mut Vec<CheckResult>) {
    for tool in ["gitleaks", "cargo-deny", "cargo-machete"] {
        if tc.is_installed(tool) {
            results.push(
                CheckResult {
                    id: ID.to_owned(),
                    severity: Severity::Error,
                    title: format!("{tool} installed"),
                    message: format!("{tool} is available on PATH for Rust hook execution."),
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
                title: format!("{tool} missing"),
                message: format!("{tool} is required by the Rust hook but is not on PATH."),
                file: Some(rel_path.to_owned()),
                line: None,
                inventory: false,
            });
        }
    }
}

#[cfg(test)]
#[path = "hook_rs_06_required_tools_installed_tests.rs"]
mod tests;

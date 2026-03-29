use guardrail3_domain_report::{CheckResult, Severity};

const ID: &str = "HOOK-SHARED-05";

pub fn check(rel_path: &str, executable: Option<bool>, results: &mut Vec<CheckResult>) {
    match executable {
        Some(true) => results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Error,
                title: "pre-commit hook is executable".to_owned(),
                message: "Dispatcher hook has the executable bit set.".to_owned(),
                file: Some(rel_path.to_owned()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        ),
        Some(false) => results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: "pre-commit hook is not executable".to_owned(),
            message: "Dispatcher hook exists but does not have the executable bit set.".to_owned(),
            file: Some(rel_path.to_owned()),
            line: None,
            inventory: false,
        }),
        None => results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: "pre-commit hook permissions unavailable".to_owned(),
            message: "Could not determine whether the dispatcher hook is executable.".to_owned(),
            file: Some(rel_path.to_owned()),
            line: None,
            inventory: false,
        }),
    }
}

#[cfg(test)]
#[path = "hook_shared_05_pre_commit_executable_tests/mod.rs"]
mod hook_shared_05_pre_commit_executable_tests;

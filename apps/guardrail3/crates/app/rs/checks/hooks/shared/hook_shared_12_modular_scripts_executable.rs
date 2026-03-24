use crate::domain::report::{CheckResult, Severity};

const ID: &str = "HOOK-SHARED-12";

pub fn check(modular_executable: &[(String, bool)], results: &mut Vec<CheckResult>) {
    if modular_executable.is_empty() {
        return;
    }

    for (rel_path, executable) in modular_executable {
        if *executable {
            results.push(
                CheckResult {
                    id: ID.to_owned(),
                    severity: Severity::Warn,
                    title: "modular hook script is executable".to_owned(),
                    message: "Modular hook script has the executable bit set.".to_owned(),
                    file: Some(rel_path.clone()),
                    line: None,
                    inventory: false,
                }
                .as_inventory(),
            );
        } else {
            results.push(CheckResult {
                id: ID.to_owned(),
                severity: Severity::Warn,
                title: "modular hook script is not executable".to_owned(),
                message: "Modular hook script exists but does not have the executable bit set."
                    .to_owned(),
                file: Some(rel_path.clone()),
                line: None,
                inventory: false,
            });
        }
    }
}

#[cfg(test)]
#[path = "hook_shared_12_modular_scripts_executable_tests.rs"]
mod tests;

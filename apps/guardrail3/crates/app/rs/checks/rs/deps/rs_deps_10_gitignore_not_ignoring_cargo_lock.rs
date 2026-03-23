use crate::domain::report::{CheckResult, Severity};

use super::inputs::LockfileDepsInput;

const ID: &str = "RS-DEPS-10";

pub fn check(input: &LockfileDepsInput<'_>, results: &mut Vec<CheckResult>) {
    if input.lockfile.cargo_lock_ignored {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: "Cargo.lock ignored in gitignore".to_owned(),
            message: "`.gitignore` must not ignore `Cargo.lock`.".to_owned(),
            file: Some(".gitignore".to_owned()),
            line: None,
            inventory: false,
        });
    } else {
        results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Info,
                title: "Cargo.lock tracked by git".to_owned(),
                message: "`.gitignore` does not ignore `Cargo.lock`.".to_owned(),
                file: Some(".gitignore".to_owned()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
    }
}

#[cfg(test)]
#[path = "rs_deps_10_gitignore_not_ignoring_cargo_lock_tests.rs"]
mod tests;

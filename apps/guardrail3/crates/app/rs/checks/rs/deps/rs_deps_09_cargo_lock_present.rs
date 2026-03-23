use crate::domain::report::{CheckResult, Severity};

use super::inputs::LockfileDepsInput;

const ID: &str = "RS-DEPS-09";

pub fn check(input: &LockfileDepsInput<'_>, results: &mut Vec<CheckResult>) {
    if input.lockfile.cargo_lock_exists {
        results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Info,
                title: "Cargo.lock committed".to_owned(),
                message: "`Cargo.lock` is committed.".to_owned(),
                file: Some("Cargo.lock".to_owned()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
        return;
    }

    let library_profile = input.lockfile.root_profile_name.as_deref() == Some("library");
    results.push(CheckResult {
        id: ID.to_owned(),
        severity: if library_profile {
            Severity::Info
        } else {
            Severity::Error
        },
        title: "Cargo.lock missing".to_owned(),
        message: if library_profile {
            "Library profile project is missing `Cargo.lock`.".to_owned()
        } else {
            "Non-library Rust project is missing `Cargo.lock`.".to_owned()
        },
        file: Some("Cargo.lock".to_owned()),
        line: None,
        inventory: false,
    });
}

#[cfg(test)]
#[path = "rs_deps_09_cargo_lock_present_tests.rs"]
mod tests;

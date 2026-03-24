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
                message: format!(
                    "Rust root `{}` has `{}` committed.",
                    rel_label(&input.lockfile.root_rel_dir),
                    input.lockfile.cargo_lock_rel_path
                ),
                file: Some(input.lockfile.cargo_lock_rel_path.clone()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
        return;
    }

    let library_profile = input.lockfile.profile_name.as_deref() == Some("library");
    results.push(CheckResult {
        id: ID.to_owned(),
        severity: if library_profile {
            Severity::Info
        } else {
            Severity::Error
        },
        title: "Cargo.lock missing".to_owned(),
        message: if library_profile {
            format!(
                "Library-profile Rust root `{}` is missing `{}`.",
                rel_label(&input.lockfile.root_rel_dir),
                input.lockfile.cargo_lock_rel_path
            )
        } else {
            format!(
                "Non-library Rust root `{}` is missing `{}`.",
                rel_label(&input.lockfile.root_rel_dir),
                input.lockfile.cargo_lock_rel_path
            )
        },
        file: Some(input.lockfile.cargo_lock_rel_path.clone()),
        line: None,
        inventory: false,
    });
}

fn rel_label(rel: &str) -> String {
    if rel.is_empty() {
        ".".to_owned()
    } else {
        rel.to_owned()
    }
}

#[cfg(test)]
#[path = "rs_deps_09_cargo_lock_present_tests/mod.rs"]
mod tests;

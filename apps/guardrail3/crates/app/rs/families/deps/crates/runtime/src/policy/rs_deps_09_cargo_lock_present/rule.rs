use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::LockfileDepsInput;

const ID: &str = "RS-DEPS-09";

pub fn check(input: &LockfileDepsInput<'_>, results: &mut Vec<CheckResult>) {
    if input.lockfile.cargo_lock_exists {
        results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Info,
                "Cargo.lock committed".to_owned(),
                format!(
                    "Rust root `{}` has `{}` committed.",
                    rel_label(&input.lockfile.root_rel_dir),
                    input.lockfile.cargo_lock_rel_path
                ),
                Some(input.lockfile.cargo_lock_rel_path.clone()),
                None,
                false,
            )
            .as_inventory(),
        );
        return;
    }

    let library_profile = input.lockfile.profile_name.as_deref() == Some("library");
    results.push(CheckResult::from_parts(
        ID.to_owned(),
        if library_profile {
            Severity::Info
        } else {
            Severity::Error
        },
        "Cargo.lock missing".to_owned(),
        if library_profile {
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
        Some(input.lockfile.cargo_lock_rel_path.clone()),
        None,
        false,
    ));
}

fn rel_label(rel: &str) -> String {
    if rel.is_empty() {
        ".".to_owned()
    } else {
        rel.to_owned()
    }
}


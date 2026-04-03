use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::LockfileDepsInput;

const ID: &str = "RS-DEPS-10";

pub fn check(input: &LockfileDepsInput<'_>, results: &mut Vec<CheckResult>) {
    if input.lockfile.cargo_lock_ignored {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            "Cargo.lock ignored in gitignore".to_owned(),
            format!(
                "`{}` ignores `{}` for Rust root `{}`. Remove the line ignoring `Cargo.lock` from this `.gitignore`.",
                input
                    .lockfile
                    .gitignore_rel_path
                    .as_deref()
                    .unwrap_or(".gitignore"),
                input.lockfile.cargo_lock_rel_path,
                rel_label(&input.lockfile.root_rel_dir)
            ),
            input.lockfile.gitignore_rel_path.clone(),
            None,
            false,
        ));
    } else {
        results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Info,
                "Cargo.lock tracked by git".to_owned(),
                format!(
                    "No relevant `.gitignore` masks `{}` for Rust root `{}`.",
                    input.lockfile.cargo_lock_rel_path,
                    rel_label(&input.lockfile.root_rel_dir)
                ),
                Some(input.lockfile.cargo_lock_rel_path.clone()),
                None,
                false,
            )
            .as_inventory(),
        );
    }
}

fn rel_label(rel: &str) -> String {
    if rel.is_empty() {
        ".".to_owned()
    } else {
        rel.to_owned()
    }
}


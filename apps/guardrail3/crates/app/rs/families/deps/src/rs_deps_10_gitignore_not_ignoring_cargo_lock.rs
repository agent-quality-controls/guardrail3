use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::LockfileDepsInput;

const ID: &str = "RS-DEPS-10";

pub fn check(input: &LockfileDepsInput<'_>, results: &mut Vec<CheckResult>) {
    if input.lockfile.cargo_lock_ignored {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: "Cargo.lock ignored in gitignore".to_owned(),
            message: format!(
                "`{}` ignores `{}` for Rust root `{}`.",
                input
                    .lockfile
                    .gitignore_rel_path
                    .as_deref()
                    .unwrap_or(".gitignore"),
                input.lockfile.cargo_lock_rel_path,
                rel_label(&input.lockfile.root_rel_dir)
            ),
            file: input.lockfile.gitignore_rel_path.clone(),
            line: None,
            inventory: false,
        });
    } else {
        results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Info,
                title: "Cargo.lock tracked by git".to_owned(),
                message: format!(
                    "No relevant `.gitignore` masks `{}` for Rust root `{}`.",
                    input.lockfile.cargo_lock_rel_path,
                    rel_label(&input.lockfile.root_rel_dir)
                ),
                file: Some(input.lockfile.cargo_lock_rel_path.clone()),
                line: None,
                inventory: false,
            }
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

#[cfg(test)]
#[path = "rs_deps_10_gitignore_not_ignoring_cargo_lock_tests/mod.rs"]
mod tests;

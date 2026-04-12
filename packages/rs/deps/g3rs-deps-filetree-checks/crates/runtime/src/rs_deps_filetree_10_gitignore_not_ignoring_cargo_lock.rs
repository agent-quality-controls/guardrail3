use g3rs_deps_filetree_checks_types::G3RsDepsFileTreeChecksInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "RS-DEPS-FILETREE-10";

pub(crate) fn check(input: &G3RsDepsFileTreeChecksInput, results: &mut Vec<G3CheckResult>) {
    if input.cargo_lock_ignored {
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "Cargo.lock ignored in gitignore".to_owned(),
            format!(
                "`{}` ignores `{}`. Remove the line ignoring `Cargo.lock` from this `.gitignore`.",
                input.gitignore_rel_path.as_deref().unwrap_or(".gitignore"),
                input.cargo_lock_rel_path
            ),
            input.gitignore_rel_path.clone(),
            None,
        ));
        return;
    }

    results.push(
        G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Info,
            "Cargo.lock tracked by git".to_owned(),
            format!(
                "No relevant `.gitignore` masks `{}` at the workspace root.",
                input.cargo_lock_rel_path
            ),
            Some(input.cargo_lock_rel_path.clone()),
            None,
        )
        .into_inventory(),
    );
}

#[cfg(test)]
#[path = "rs_deps_filetree_10_gitignore_not_ignoring_cargo_lock_tests/mod.rs"]
mod tests;

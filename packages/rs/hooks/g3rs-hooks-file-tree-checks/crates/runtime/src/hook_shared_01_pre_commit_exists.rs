use g3rs_hooks_file_tree_checks_types::G3RsHooksScriptFileFact;
use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "RS-HOOKS-FILETREE-01";

pub(crate) fn check(input: Option<&G3RsHooksScriptFileFact>, results: &mut Vec<G3CheckResult>) {
    match input {
        Some(script) => results.push(
            G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Error,
                "pre-commit hook exists".to_owned(),
                "Found cached pre-commit hook.".to_owned(),
                Some(script.rel_path.clone()),
                None,
            )
            .into_inventory(),
        ),
        None => results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "pre-commit hook missing".to_owned(),
            "Expected a cached `.githooks/pre-commit` or `hooks/pre-commit` hook.".to_owned(),
            Some(".githooks/pre-commit".to_owned()),
            None,
        )),
    }
}

#[cfg(test)]
#[path = "hook_shared_01_pre_commit_exists_tests/mod.rs"]
mod tests;

use g3rs_hooks_types::G3RsHooksScriptFileFact;
use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "g3rs-hooks/pre-commit-executable";

pub(crate) fn check(input: &G3RsHooksScriptFileFact, results: &mut Vec<G3CheckResult>) {
    match input.executable {
        Some(true) => results.push(
            G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Error,
                "pre-commit hook is executable".to_owned(),
                "Dispatcher hook has the executable bit set.".to_owned(),
                Some(input.rel_path.clone()),
                None,
            )
            .into_inventory(),
        ),
        Some(false) => results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "pre-commit hook is not executable".to_owned(),
            "Dispatcher hook exists but does not have the executable bit set.".to_owned(),
            Some(input.rel_path.clone()),
            None,
        )),
        None => results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "pre-commit hook permissions unavailable".to_owned(),
            "Could not determine whether the dispatcher hook is executable.".to_owned(),
            Some(input.rel_path.clone()),
            None,
        )),
    }
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod rule_tests;

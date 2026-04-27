use g3rs_hooks_types::G3RsHooksScriptFileFact;
use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "g3rs-hooks/modular-scripts-inventory";

pub(crate) fn check(scripts: &[G3RsHooksScriptFileFact], results: &mut Vec<G3CheckResult>) {
    if scripts.is_empty() {
        results.push(
            G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Info,
                "no modular hook scripts".to_owned(),
                "No cached files found in `.githooks/pre-commit.d`.".to_owned(),
                Some(".githooks/pre-commit.d".to_owned()),
                None,
            )
            .into_inventory(),
        );
        return;
    }

    let names = scripts
        .iter()
        .map(|script| script.rel_path.clone())
        .collect::<Vec<_>>()
        .join(", ");
    results.push(
        G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Info,
            "modular hook scripts inventory".to_owned(),
            names,
            Some(".githooks/pre-commit.d".to_owned()),
            None,
        )
        .into_inventory(),
    );
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod rule_tests;

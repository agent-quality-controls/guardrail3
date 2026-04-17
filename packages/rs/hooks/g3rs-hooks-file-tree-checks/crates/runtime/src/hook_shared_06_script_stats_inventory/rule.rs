use g3rs_hooks_types::G3RsHooksScriptFileFact;
use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "RS-HOOKS-FILETREE-09";

pub(crate) fn check(input: &G3RsHooksScriptFileFact, results: &mut Vec<G3CheckResult>) {
    results.push(
        G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Info,
            "pre-commit script stats".to_owned(),
            format!("{} lines, {} bytes", input.line_count, input.byte_count),
            Some(input.rel_path.clone()),
            None,
        )
        .into_inventory(),
    );
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod rule_tests;

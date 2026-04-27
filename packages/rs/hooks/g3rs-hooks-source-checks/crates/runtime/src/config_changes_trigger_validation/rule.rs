use crate::compat::{G3CheckResult, G3Severity};
use crate::inputs::RustHookCommandInput;

use super::support::missing_config_needles;

const ID: &str = "g3rs-hooks/config-changes-trigger-validation";

pub(crate) fn check(input: &RustHookCommandInput<'_>, results: &mut Vec<G3CheckResult>) {
    let missing_needles = missing_config_needles(input.parsed);

    if missing_needles.is_empty() {
        results.push(
            G3CheckResult::from_parts(
                ID.to_owned(),
                G3Severity::Warn,
                "`.githooks/pre-commit` triggers Rust validation on guardrail config changes"
                    .to_owned(),
                "`.githooks/pre-commit` includes Rust guardrail config files in the staged-file trigger logic that decides when to run Rust validation.".to_owned(),
                Some(input.rel_path.to_owned()),
                None,
                false,
            )
            .into_inventory(),
        );
    } else {
        results.push(G3CheckResult::from_parts(
            ID.to_owned(),
            G3Severity::Warn,
            "incomplete Rust guardrail config trigger coverage in `.githooks/pre-commit`"
                .to_owned(),
            format!(
                "The staged-file trigger logic in `.githooks/pre-commit` does not clearly match these Rust guardrail config files: {}. Add them to the condition that decides when to run Rust validation so a config-only policy change cannot bypass the hook.",
                missing_needles.join(", ")
            ),
            Some(input.rel_path.to_owned()),
            None,
            false,
        ));
    }
}

#[cfg(test)]
pub(crate) fn run_case(content: &str) -> Vec<guardrail3_check_types::G3CheckResult> {
    let parsed = hook_shell_parser::parse_script(content);
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
        is_workspace_project: true,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    crate::compat::finish(results)
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod rule_tests;

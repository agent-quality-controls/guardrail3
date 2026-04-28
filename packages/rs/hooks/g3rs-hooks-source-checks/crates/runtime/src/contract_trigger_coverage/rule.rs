use std::collections::BTreeSet;

use crate::compat::{G3CheckResult, G3Severity};
use crate::inputs::RustHookCommandInput;
use g3rs_hooks_contract_types::G3HookTriggerPattern;

const ID: &str = "g3rs-hooks/contract-trigger-coverage";

pub(crate) fn check(input: &RustHookCommandInput<'_>, results: &mut Vec<G3CheckResult>) {
    let exact_paths = exact_trigger_paths(input);
    if exact_paths.is_empty() {
        results.push(
            G3CheckResult::from_parts(
                ID.to_owned(),
                G3Severity::Warn,
                "hook contract has no exact trigger paths to prove".to_owned(),
                "Family hook contracts only declare glob or extension trigger patterns here. G3RS cannot yet prove glob trigger routing, so legacy hook rules remain active.".to_owned(),
                Some(input.rel_path.to_owned()),
                None,
                false,
            )
            .into_inventory(),
        );
        return;
    }

    results.push(G3CheckResult::from_parts(
        ID.to_owned(),
        G3Severity::Warn,
        "hook contract trigger coverage is not proven".to_owned(),
        format!(
            "G3RS cannot yet prove that staged-file dispatch routes these family-owned paths to `g3rs validate --path ...`: {}. Keep the legacy hook trigger checks active until this rule can model shell conditions without raw text matching.",
            exact_paths.join(", ")
        ),
        Some(input.rel_path.to_owned()),
        None,
        false,
    ));
}

fn exact_trigger_paths(input: &RustHookCommandInput<'_>) -> Vec<String> {
    let mut paths = BTreeSet::new();
    for requirement in input.requirements {
        for pattern in &requirement.trigger_patterns {
            if let G3HookTriggerPattern::ExactPath(path) = pattern {
                let _ = paths.insert(path.clone());
            }
        }
    }
    paths.into_iter().collect()
}

#[cfg(test)]
pub(crate) fn run_case(
    content: &str,
    requirements: Vec<g3rs_hooks_contract_types::G3HookRequirement>,
) -> Vec<guardrail3_check_types::G3CheckResult> {
    let parsed = hook_shell_parser::parse_script(content);
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
        is_workspace_project: true,
        requirements: &requirements,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    crate::compat::finish(results)
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"]
mod rule_tests;

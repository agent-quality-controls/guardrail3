use g3ts_topology_types::G3TsTopologyNestedGuardrail3TsTomlInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::support::display_dir;

/// Stable rule identifier emitted on every check result.
const ID: &str = "g3ts-topology/no-nested-guardrail3-ts-toml";

/// Emits an error result for a nested `guardrail3-ts.toml` fact.
pub(crate) fn check(
    input: &G3TsTopologyNestedGuardrail3TsTomlInput,
    results: &mut Vec<G3CheckResult>,
) {
    let parent_unit_rel = &input.parent_unit_rel;

    results.push(G3CheckResult::new(
        ID.to_owned(),
        G3Severity::Error,
        format!(
            "Nested `guardrail3-ts.toml` at `{}` is forbidden",
            display_dir(&input.rel_dir)
        ),
        format!(
            "`{}` declares a nested adopted TS unit marker under `{}`. Nested adoption breaks the upward-walk routing assumption. Remove the inner `{}`, or move it so it is not nested under `{}`.",
            input.toml_rel_path,
            display_dir(parent_unit_rel),
            input.toml_rel_path,
            display_dir(parent_unit_rel),
        ),
        Some(input.toml_rel_path.clone()),
        None,
    ));
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod rule_tests;

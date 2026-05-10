use g3rs_topology_types::G3RsTopologyNestedGuardrail3RsTomlInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::support::display_dir;

/// Stable identifier for this rule.
const ID: &str = "g3rs-topology/no-nested-guardrail3-rs-toml";

/// Runs this rule and appends its findings to `results`.
pub(crate) fn check(
    input: &G3RsTopologyNestedGuardrail3RsTomlInput,
    results: &mut Vec<G3CheckResult>,
) {
    let outer_label = display_dir(&input.outer_adopted_unit_rel);

    results.push(G3CheckResult::new(
        ID.to_owned(),
        G3Severity::Error,
        format!(
            "Nested adopted Rust unit `{}` is forbidden",
            display_dir(&input.rel_dir)
        ),
        format!(
            "`{}` declares a nested adopted Rust unit under `{outer_label}`. Nested guardrail3 adoption breaks ownership routing. Remove `{}`, or move the inner unit so it is not nested under `{outer_label}`.",
            input.guardrail3_rs_toml_rel_path,
            input.guardrail3_rs_toml_rel_path,
        ),
        Some(input.guardrail3_rs_toml_rel_path.clone()),
        None,
    ));
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod rule_tests;

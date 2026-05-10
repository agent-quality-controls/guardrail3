use g3rs_topology_types::G3RsTopologyNestedWorkspaceInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::support::display_dir;

/// Stable identifier for this rule.
const ID: &str = "g3rs-topology/no-nested-workspaces";

/// Runs this rule and appends its findings to `results`.
pub(crate) fn check(input: &G3RsTopologyNestedWorkspaceInput, results: &mut Vec<G3CheckResult>) {
    let parent_workspace_rel = &input.parent_workspace_rel;

    results.push(G3CheckResult::new(
        ID.to_owned(),
        G3Severity::Error,
        format!(
            "Nested workspace `{}` is forbidden",
            display_dir(&input.rel_dir)
        ),
        format!(
            "`{}` declares a nested workspace under `{}`. Cargo does not support nested workspaces. Remove the `[workspace]` section from this Cargo.toml, or move it so it is not nested under `{}`.",
            input.cargo_rel_path,
            display_dir(parent_workspace_rel),
            display_dir(parent_workspace_rel),
        ),
        Some(input.cargo_rel_path.clone()),
        None,
    ));
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod rule_tests;

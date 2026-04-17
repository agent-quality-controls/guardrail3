use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::support::display_dir;

const ID: &str = "RS-TOPOLOGY-FILETREE-11";

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct NestedWorkspaceInput {
    pub(crate) rel_dir: String,
    pub(crate) cargo_rel_path: String,
    pub(crate) parent_workspace_rel: String,
}

pub(crate) fn check(input: &NestedWorkspaceInput, results: &mut Vec<G3CheckResult>) {
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

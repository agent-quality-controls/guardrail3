use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::support::{TopologyIssue, TopologyIssueKind, display_dir};

const ID: &str = "RS-TOPOLOGY-FILETREE-11";

pub(crate) fn check(input: &TopologyIssue, results: &mut Vec<G3CheckResult>) {
    let TopologyIssueKind::NestedWorkspace {
        parent_workspace_rel,
    } = &input.kind
    else {
        return;
    };

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
#[path = "rs_topology_11_no_nested_workspaces_tests/mod.rs"]
mod tests;

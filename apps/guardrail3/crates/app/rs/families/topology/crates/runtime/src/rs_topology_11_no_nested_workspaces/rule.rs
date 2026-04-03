use guardrail3_domain_report::{CheckResult, Severity};

use crate::facts::TopologyIssueKind;
use crate::inputs::TopologyIssueInput;

const ID: &str = "RS-TOPOLOGY-11";

pub fn check(input: &TopologyIssueInput<'_>, results: &mut Vec<CheckResult>) {
    let TopologyIssueKind::NestedWorkspace {
        parent_workspace_rel,
    } = &input.issue.kind
    else {
        return;
    };

    results.push(CheckResult::from_parts(
        ID.to_owned(),
        Severity::Error,
        format!(
            "Nested workspace `{}` is forbidden",
            display_dir(&input.issue.rel_dir)
        ),
        format!(
            "`{}` declares a nested workspace under `{}`. Cargo does not support nested workspaces. Remove the `[workspace]` section from this Cargo.toml, or move it so it is not nested under `{}`.",
            input.issue.cargo_rel_path,
            display_dir(parent_workspace_rel),
            display_dir(parent_workspace_rel),
        ),
        Some(input.issue.cargo_rel_path.clone()),
        None,
        false,
    ));
}

fn display_dir(rel_dir: &str) -> &str {
    if rel_dir.is_empty() { "." } else { rel_dir }
}


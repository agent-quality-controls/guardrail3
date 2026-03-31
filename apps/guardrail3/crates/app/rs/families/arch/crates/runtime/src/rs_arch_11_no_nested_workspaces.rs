use guardrail3_domain_report::{CheckResult, Severity};

use super::facts::ArchTopologyIssueKind;
use super::inputs::TopologyIssueInput;

const ID: &str = "RS-ARCH-11";

pub fn check(input: &TopologyIssueInput<'_>, results: &mut Vec<CheckResult>) {
    let ArchTopologyIssueKind::NestedWorkspace {
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
            "`{}` declares a nested workspace under `{}`. Nested workspaces are forbidden.",
            input.issue.cargo_rel_path,
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

#[cfg(test)]
#[path = "rs_arch_11_no_nested_workspaces_tests/mod.rs"]
mod rs_arch_11_no_nested_workspaces_tests;

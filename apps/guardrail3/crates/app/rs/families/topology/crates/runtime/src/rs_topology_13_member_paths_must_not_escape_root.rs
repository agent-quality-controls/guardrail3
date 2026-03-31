use guardrail3_domain_report::{CheckResult, Severity};

use super::facts::TopologyIssueKind;
use super::inputs::TopologyIssueInput;

const ID: &str = "RS-TOPOLOGY-13";

pub fn check(input: &TopologyIssueInput<'_>, results: &mut Vec<CheckResult>) {
    let TopologyIssueKind::WorkspaceMemberPathEscapesRoot {
        workspace_root_rel,
        member_pattern,
    } = &input.issue.kind
    else {
        return;
    };

    results.push(CheckResult::from_parts(
        ID.to_owned(),
        Severity::Error,
        format!(
            "Workspace `{}` uses escaping member path `{member_pattern}`",
            display_dir(workspace_root_rel)
        ),
        format!(
            "`{}` declares member pattern `{member_pattern}`, which escapes the workspace root.",
            input.issue.cargo_rel_path,
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
#[path = "rs_topology_13_member_paths_must_not_escape_root_tests/mod.rs"]
mod rs_topology_13_member_paths_must_not_escape_root_tests;

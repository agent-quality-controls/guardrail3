use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::support::{TopologyIssue, TopologyIssueKind, display_dir};

const ID: &str = "RS-TOPOLOGY-13";

pub(crate) fn check(input: &TopologyIssue, results: &mut Vec<G3CheckResult>) {
    let TopologyIssueKind::WorkspaceMemberPathEscapesRoot {
        workspace_root_rel,
        member_pattern,
    } = &input.kind
    else {
        return;
    };

    results.push(G3CheckResult::new(
        ID.to_owned(),
        G3Severity::Error,
        format!(
            "Workspace `{}` uses escaping member path `{member_pattern}`",
            display_dir(workspace_root_rel)
        ),
        format!(
            "`{}` declares member pattern `{member_pattern}`, which points outside the workspace directory. Workspace members must be relative subdirectory paths inside the workspace root, not absolute paths or `..` escapes. Change the pattern to a relative subdirectory path, or move the target crate inside the workspace.",
            input.cargo_rel_path,
        ),
        Some(input.cargo_rel_path.clone()),
        None,
    ));
}

#[cfg(test)]
#[path = "rs_topology_13_member_paths_must_not_escape_root_tests/mod.rs"]
mod tests;

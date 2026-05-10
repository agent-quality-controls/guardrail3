use g3rs_topology_types::G3RsTopologyEscapingWorkspaceMemberPathInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::support::display_dir;

/// Stable identifier for this rule.
const ID: &str = "g3rs-topology/member-paths-must-not-escape-root";

/// Runs this rule and appends its findings to `results`.
pub(crate) fn check(
    input: &G3RsTopologyEscapingWorkspaceMemberPathInput,
    results: &mut Vec<G3CheckResult>,
) {
    let workspace_root_rel = &input.workspace_root_rel;
    let member_pattern = &input.member_pattern;

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
#[path = "rule_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod rule_tests;

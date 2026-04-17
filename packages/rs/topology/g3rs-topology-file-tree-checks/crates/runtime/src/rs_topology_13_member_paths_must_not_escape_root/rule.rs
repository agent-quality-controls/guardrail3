use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::support::display_dir;

const ID: &str = "RS-TOPOLOGY-FILETREE-13";

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct EscapingWorkspaceMemberPathInput {
    pub(crate) cargo_rel_path: String,
    pub(crate) workspace_root_rel: String,
    pub(crate) member_pattern: String,
}

pub(crate) fn check(input: &EscapingWorkspaceMemberPathInput, results: &mut Vec<G3CheckResult>) {
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

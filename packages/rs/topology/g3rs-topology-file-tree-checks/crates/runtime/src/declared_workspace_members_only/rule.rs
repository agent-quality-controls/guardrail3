use g3rs_topology_types::{
    G3RsTopologyWorkspaceMemberIssueInput, G3RsTopologyWorkspaceMemberIssueKind,
};
use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::support::display_dir;

/// Stable identifier for this rule.
const ID: &str = "g3rs-topology/declared-workspace-members-only";

/// Runs this rule and appends its findings to `results`.
pub(crate) fn check(
    input: &G3RsTopologyWorkspaceMemberIssueInput,
    results: &mut Vec<G3CheckResult>,
) {
    match &input.kind {
        G3RsTopologyWorkspaceMemberIssueKind::Undeclared { workspace_root_rel } => {
            results.push(G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Error,
                format!(
                    "Workspace child `{}` must be declared explicitly",
                    display_dir(&input.rel_dir)
                ),
                format!(
                    "`{}` sits under workspace `{}`, but it is not a declared workspace member. Workspace membership must exactly match real child Rust roots. Add this crate's path to `[workspace] members` in `{}/Cargo.toml`.",
                    input.cargo_rel_path,
                    display_dir(workspace_root_rel),
                    display_dir(workspace_root_rel),
                ),
                Some(input.cargo_rel_path.clone()),
                None,
            ));
        }
        G3RsTopologyWorkspaceMemberIssueKind::Extra {
            workspace_root_rel,
            member_pattern,
        } => {
            results.push(G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Error,
                format!(
                    "Workspace `{}` has extra member `{member_pattern}`",
                    display_dir(workspace_root_rel)
                ),
                format!(
                    "`{}` declares workspace member `{member_pattern}`, but it does not match a real owned child Rust root. Workspace membership must exactly match real child Rust roots. Remove `{member_pattern}` from `[workspace] members`, or create the missing crate.",
                    input.cargo_rel_path,
                ),
                Some(input.cargo_rel_path.clone()),
                None,
            ));
        }
    }
}

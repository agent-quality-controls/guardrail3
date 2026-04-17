use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::support::display_dir;

const ID: &str = "RS-TOPOLOGY-FILETREE-12";

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum WorkspaceMemberIssueKind {
    Undeclared {
        workspace_root_rel: String,
    },
    Extra {
        workspace_root_rel: String,
        member_pattern: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct WorkspaceMemberIssueInput {
    pub(crate) rel_dir: String,
    pub(crate) cargo_rel_path: String,
    pub(crate) kind: WorkspaceMemberIssueKind,
}

pub(crate) fn check(input: &WorkspaceMemberIssueInput, results: &mut Vec<G3CheckResult>) {
    match &input.kind {
        WorkspaceMemberIssueKind::Undeclared { workspace_root_rel } => {
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
        WorkspaceMemberIssueKind::Extra {
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

#[cfg(test)]
#[path = "rule_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod rule_tests;

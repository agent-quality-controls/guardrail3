use guardrail3_domain_report::{CheckResult, Severity};

use super::facts::ArchTopologyIssueKind;
use super::inputs::TopologyIssueInput;

const ID: &str = "RS-ARCH-12";

pub fn check(input: &TopologyIssueInput<'_>, results: &mut Vec<CheckResult>) {
    match &input.issue.kind {
        ArchTopologyIssueKind::UndeclaredWorkspaceMember { workspace_root_rel } => {
            results.push(CheckResult::from_parts(
                ID.to_owned(),
                Severity::Error,
                format!(
                    "Workspace child `{}` must be declared explicitly",
                    display_dir(&input.issue.rel_dir)
                ),
                format!(
                    "`{}` sits under workspace `{}`, but it is not a declared workspace member. Workspace membership must exactly match real child Rust roots.",
                    input.issue.cargo_rel_path,
                    display_dir(workspace_root_rel),
                ),
                Some(input.issue.cargo_rel_path.clone()),
                None,
                false,
            ));
        }
        ArchTopologyIssueKind::ExtraWorkspaceMember {
            workspace_root_rel,
            member_pattern,
        } => {
            results.push(CheckResult::from_parts(
                ID.to_owned(),
                Severity::Error,
                format!(
                    "Workspace `{}` has extra member `{member_pattern}`",
                    display_dir(workspace_root_rel)
                ),
                format!(
                    "`{}` declares workspace member `{member_pattern}`, but it does not match a real owned child Rust root. Workspace membership must exactly match real child Rust roots.",
                    input.issue.cargo_rel_path,
                ),
                Some(input.issue.cargo_rel_path.clone()),
                None,
                false,
            ));
        }
        _ => {}
    }
}

fn display_dir(rel_dir: &str) -> &str {
    if rel_dir.is_empty() { "." } else { rel_dir }
}

#[cfg(test)]
#[path = "rs_arch_12_declared_workspace_members_only_tests/mod.rs"]
mod rs_arch_12_declared_workspace_members_only_tests;

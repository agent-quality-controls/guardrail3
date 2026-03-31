use guardrail3_domain_report::{CheckResult, Severity};

use super::facts::ArchTopologyIssueKind;
use super::inputs::TopologyIssueInput;

const ID: &str = "RS-ARCH-12";

pub fn check(input: &TopologyIssueInput<'_>, results: &mut Vec<CheckResult>) {
    let ArchTopologyIssueKind::UndeclaredWorkspaceMember {
        workspace_root_rel,
    } = &input.issue.kind
    else {
        return;
    };

    results.push(CheckResult::from_parts(
        ID.to_owned(),
        Severity::Error,
        format!(
            "Workspace member `{}` must be declared explicitly",
            display_dir(&input.issue.rel_dir)
        ),
        format!(
            "`{}` sits under workspace `{}`, but it is not a declared workspace member.",
            input.issue.cargo_rel_path,
            display_dir(workspace_root_rel),
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
#[path = "rs_arch_12_declared_workspace_members_only_tests/mod.rs"]
mod rs_arch_12_declared_workspace_members_only_tests;

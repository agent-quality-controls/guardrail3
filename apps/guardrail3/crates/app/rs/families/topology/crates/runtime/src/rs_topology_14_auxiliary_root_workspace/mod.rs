use guardrail3_domain_report::{CheckResult, Severity};

use super::facts::TopologyIssueKind;
use super::inputs::TopologyIssueInput;

const ID: &str = "RS-TOPOLOGY-14";

pub fn check(input: &TopologyIssueInput<'_>, results: &mut Vec<CheckResult>) {
    if !matches!(
        input.issue.kind,
        TopologyIssueKind::AuxiliaryTopLevelRootMustBeWorkspace
    ) {
        return;
    }

    results.push(CheckResult::from_parts(
        ID.to_owned(),
        Severity::Error,
        format!(
            "Auxiliary top-level Rust root `{}` must be a workspace",
            display_dir(&input.issue.rel_dir)
        ),
        format!(
            "`{}` is an auxiliary top-level Rust root. Auxiliary roots must still declare `[workspace]`.",
            input.issue.cargo_rel_path
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

mod rs_topology_14_auxiliary_root_workspace_tests;

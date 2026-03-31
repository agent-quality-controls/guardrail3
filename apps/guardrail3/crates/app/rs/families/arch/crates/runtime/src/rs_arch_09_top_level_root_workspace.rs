use guardrail3_domain_report::{CheckResult, Severity};

use super::facts::ArchTopologyIssueKind;
use super::inputs::TopologyIssueInput;

const ID: &str = "RS-ARCH-09";

pub fn check(input: &TopologyIssueInput<'_>, results: &mut Vec<CheckResult>) {
    if !matches!(
        input.issue.kind,
        ArchTopologyIssueKind::TopLevelRootMustBeWorkspace
    ) {
        return;
    }

    results.push(CheckResult::from_parts(
        ID.to_owned(),
        Severity::Error,
        format!(
            "Top-level Rust root `{}` must be a workspace",
            display_dir(&input.issue.rel_dir)
        ),
        format!(
            "`{}` is a top-level Rust root. Top-level Rust roots must declare `[workspace]`.",
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
#[path = "rs_arch_09_top_level_root_workspace_tests/mod.rs"]
mod rs_arch_09_top_level_root_workspace_tests;

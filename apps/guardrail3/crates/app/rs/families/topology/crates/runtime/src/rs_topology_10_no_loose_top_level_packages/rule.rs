use guardrail3_domain_report::{CheckResult, Severity};

use crate::facts::TopologyIssueKind;
use crate::inputs::TopologyIssueInput;

const ID: &str = "RS-TOPOLOGY-10";

pub fn check(input: &TopologyIssueInput<'_>, results: &mut Vec<CheckResult>) {
    if !matches!(
        input.issue.kind,
        TopologyIssueKind::LooseTopLevelPackage
    ) {
        return;
    }

    results.push(CheckResult::from_parts(
        ID.to_owned(),
        Severity::Error,
        format!(
            "Loose top-level package `{}` is forbidden",
            display_dir(&input.issue.rel_dir)
        ),
        format!(
            "`{}` declares `[package]` as a top-level Rust root. Top-level Rust code must live in workspaces, not loose packages.",
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


use guardrail3_domain_report::{CheckResult, Severity};

use crate::facts::TopologyIssueKind;
use crate::inputs::TopologyIssueInput;

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
            "`{}` is a top-level Rust root marked `topology_role = \"auxiliary\"`. Even auxiliary roots must declare `[workspace]`. Add a `[workspace]` section to this Cargo.toml.",
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


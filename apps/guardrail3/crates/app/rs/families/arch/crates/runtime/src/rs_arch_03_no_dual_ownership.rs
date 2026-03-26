use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::DualOwnershipInput;

const ID: &str = "RS-ARCH-03";

pub fn check(input: &DualOwnershipInput<'_>, results: &mut Vec<CheckResult>) {
    if input.root.owner_families.len() < 2 {
        return;
    }

    results.push(CheckResult {
        id: ID.to_owned(),
        severity: Severity::Error,
        title: format!(
            "Rust root `{}` has dual architecture ownership",
            display_dir(&input.root.rel_dir)
        ),
        message: format!(
            "`{}` is simultaneously owned by app zone(s) [{}] and package zone(s) [{}]. A single Rust root must not be governed by both hexarch and libarch.",
            input.root.cargo_rel_path,
            input.root.app_zone_candidates.join(", "),
            input.root.package_zone_candidates.join(", "),
        ),
        file: Some(input.root.cargo_rel_path.clone()),
        line: None,
        inventory: false,
    });
}

fn display_dir(rel_dir: &str) -> &str {
    if rel_dir.is_empty() { "." } else { rel_dir }
}

#[cfg(test)]
pub(crate) fn check_results(
    tree: &guardrail3_domain_project_tree::ProjectTree,
) -> Vec<guardrail3_domain_report::CheckResult> {
    crate::check_test_tree(tree)
}

#[cfg(test)]
#[path = "rs_arch_03_no_dual_ownership_tests/mod.rs"]
mod rs_arch_03_no_dual_ownership_tests;

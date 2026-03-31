use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::DualOwnershipInput;

const ID: &str = "RS-ARCH-03";

pub fn check(input: &DualOwnershipInput<'_>, results: &mut Vec<CheckResult>) {
    if input.root.owner_families.len() >= 2 {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            format!(
                "Rust root `{}` has dual architecture ownership",
                display_dir(&input.root.rel_dir)
            ),
            format!(
                "`{}` is simultaneously owned by app zone(s) [{}] and package zone(s) [{}]. A single Rust root must not be governed by both hexarch and libarch.",
                input.root.cargo_rel_path,
                input.root.app_zone_candidates.join(", "),
                input.root.package_zone_candidates.join(", "),
            ),
            Some(input.root.cargo_rel_path.clone()),
            None,
            false,
        ));
        return;
    }

    results.push(
        CheckResult::from_parts(
            ID.to_owned(),
            Severity::Info,
            format!(
                "Rust root `{}` avoids dual architecture ownership",
                display_dir(&input.root.rel_dir)
            ),
            format!(
                "`{}` is governed by at most one architecture family.",
                input.root.cargo_rel_path
            ),
            Some(input.root.cargo_rel_path.clone()),
            None,
            false,
        )
        .as_inventory(),
    );
}

fn display_dir(rel_dir: &str) -> &str {
    if rel_dir.is_empty() { "." } else { rel_dir }
}

#[cfg(test)]
pub(crate) fn check_results(
    tree: &guardrail3_app_rs_family_mapper::RsProjectSurface,
) -> Vec<guardrail3_domain_report::CheckResult> {
    crate::check_test_tree(tree)
}

#[cfg(test)]
#[path = "rs_arch_03_no_dual_ownership_tests/mod.rs"]
mod rs_arch_03_no_dual_ownership_tests;

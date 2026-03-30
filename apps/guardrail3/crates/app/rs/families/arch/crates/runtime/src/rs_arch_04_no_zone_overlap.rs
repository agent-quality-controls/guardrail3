use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::ZoneOverlapInput;

const ID: &str = "RS-ARCH-04";

pub fn check(input: &ZoneOverlapInput<'_>, results: &mut Vec<CheckResult>) {
    let (nesting_message, file) = if input
        .overlap
        .package_root_rel
        .starts_with(&format!("{}/", input.overlap.app_root_rel))
    {
        (
            format!(
                "package root `{}` nests inside app root `{}`",
                input.overlap.package_root_rel, input.overlap.app_root_rel
            ),
            input.overlap.package_cargo_rel_path.clone(),
        )
    } else {
        (
            format!(
                "app root `{}` nests inside package root `{}`",
                input.overlap.app_root_rel, input.overlap.package_root_rel
            ),
            input.overlap.app_cargo_rel_path.clone(),
        )
    };

    results.push(CheckResult::from_parts(
        ID.to_owned(),
        Severity::Error,
        "app and package architecture zones overlap illegally".to_owned(),
        format!(
            "{nesting_message}. app Cargo root: `{}`; package Cargo root: `{}`. App/package architecture zones must not overlap or nest.",
            input.overlap.app_cargo_rel_path, input.overlap.package_cargo_rel_path
        ),
        Some(file),
        None,
        false,
    ));
}

pub fn check_success(has_overlaps: bool, results: &mut Vec<CheckResult>) {
    if has_overlaps {
        return;
    }

    results.push(
        CheckResult::from_parts(
            ID.to_owned(),
            Severity::Info,
            "No illegal app/package zone overlap found".to_owned(),
            "App and package architecture zones do not overlap or nest illegally."
                .to_owned(),
            None,
            None,
            false,
        )
        .as_inventory(),
    );
}

#[cfg(test)]
pub(crate) fn check_results(
    tree: &guardrail3_domain_project_tree::ProjectTree,
) -> Vec<guardrail3_domain_report::CheckResult> {
    crate::check_test_tree(tree)
}

#[cfg(test)]
#[path = "rs_arch_04_no_zone_overlap_tests/mod.rs"]
mod rs_arch_04_no_zone_overlap_tests;

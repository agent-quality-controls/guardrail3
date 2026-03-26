use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::ZoneOverlapInput;

const ID: &str = "RS-ARCH-04";

pub fn check(input: &ZoneOverlapInput<'_>, results: &mut Vec<CheckResult>) {
    let nesting_message = if input
        .overlap
        .package_root_rel
        .starts_with(&format!("{}/", input.overlap.app_root_rel))
    {
        format!(
            "package root `{}` nests inside app root `{}`",
            input.overlap.package_root_rel, input.overlap.app_root_rel
        )
    } else {
        format!(
            "app root `{}` nests inside package root `{}`",
            input.overlap.app_root_rel, input.overlap.package_root_rel
        )
    };

    results.push(CheckResult {
        id: ID.to_owned(),
        severity: Severity::Error,
        title: "app and package architecture zones overlap illegally".to_owned(),
        message: format!(
            "{nesting_message}. app Cargo root: `{}`; package Cargo root: `{}`. App/package architecture zones must not overlap or nest.",
            input.overlap.app_cargo_rel_path, input.overlap.package_cargo_rel_path
        ),
        file: Some(input.overlap.package_cargo_rel_path.clone()),
        line: None,
        inventory: false,
    });
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

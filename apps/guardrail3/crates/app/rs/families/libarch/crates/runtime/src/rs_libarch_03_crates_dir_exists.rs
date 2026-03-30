use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::PackageLibarchInput;

const ID: &str = "RS-LIBARCH-03";

pub fn check(input: &PackageLibarchInput<'_>, results: &mut Vec<CheckResult>) {
    let package = input.package;
    if !package.layered_rules_active() || package.crates_dir_exists {
        return;
    }

    results.push(CheckResult::from_parts(
        ID.to_owned(),
        Severity::Error,
        "Layered library root must contain crates/".to_owned(),
        format!(
            "Layered library `{}` must define `crates/api` and `crates/core` beneath `{}`.",
            package.package_rel_dir,
            format_args!("{}/crates", package.package_rel_dir)
        ),
        Some(package.cargo_rel_path.clone()),
        None,
        false,
    ));
}

#[cfg(test)]
pub(super) fn run_family_check(root: &std::path::Path) -> Vec<CheckResult> {
    crate::check_test_root(root)
}

#[cfg(test)]
#[path = "rs_libarch_03_crates_dir_exists_tests/mod.rs"] // reason: test-only sidecar module wiring
mod rs_libarch_03_crates_dir_exists_tests;

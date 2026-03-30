use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::PackageLibarchInput;

const ID: &str = "RS-LIBARCH-02";

pub fn check(input: &PackageLibarchInput<'_>, results: &mut Vec<CheckResult>) {
    let package = input.package;
    if !package.layered_rules_active() {
        return;
    }

    if let Some(error) = &package.cargo_parse_error {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            "layered library root must parse as workspace facade".to_owned(),
            format!(
                "Package `{}` cannot prove layered root semantics because `{}` could not be parsed: {error}",
                package.package_rel_dir, package.cargo_rel_path
            ),
            Some(package.cargo_rel_path.clone()),
            None,
            false,
        ));
        return;
    }

    if !package.is_workspace || !package.has_package || !package.is_library {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            "layered root must be workspace and facade package".to_owned(),
            format!(
                "Layered library root `{}` must keep both `[workspace]` and `[package]`, and it must remain a library facade package.",
                package.package_rel_dir
            ),
            Some(package.cargo_rel_path.clone()),
            None,
            false,
        ));
        return;
    }

    results.push(
        CheckResult::from_parts(
            ID.to_owned(),
            Severity::Info,
            "layered root is workspace facade".to_owned(),
            format!(
                "Layered library root `{}` is both a workspace root and a facade package.",
                package.package_rel_dir
            ),
            Some(package.cargo_rel_path.clone()),
            None,
            false,
        )
        .as_inventory(),
    );
}

#[cfg(test)]
pub(super) fn run_family_check(root: &std::path::Path) -> Vec<CheckResult> {
    crate::check_test_root(root)
}

#[cfg(test)]
#[path = "rs_libarch_02_layered_root_workspace_facade_tests/mod.rs"] // reason: test-only sidecar module wiring
mod rs_libarch_02_layered_root_workspace_facade_tests;

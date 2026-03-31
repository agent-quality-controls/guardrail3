use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::PackageArchInput;

const ID: &str = "RS-ARCH-02";

pub fn check(input: &PackageArchInput<'_>, results: &mut Vec<CheckResult>) {
    let package = input.package;
    if !package.split_rules_active() {
        return;
    }

    if let Some(error) = &package.cargo_parse_error {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            "split library root must parse as workspace facade".to_owned(),
            format!(
                "Library `{}` cannot prove split-root semantics because `{}` could not be parsed: {error}",
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
            "split root must be workspace facade package".to_owned(),
            format!(
                "Split library root `{}` must keep both `[workspace]` and `[package]`, and it must remain a library facade package.",
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
            "split root is workspace facade".to_owned(),
            format!(
                "Split library root `{}` is both a workspace root and a facade package.",
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
#[path = "rs_arch_02_split_root_workspace_facade_tests/mod.rs"]
mod rs_arch_02_split_root_workspace_facade_tests;

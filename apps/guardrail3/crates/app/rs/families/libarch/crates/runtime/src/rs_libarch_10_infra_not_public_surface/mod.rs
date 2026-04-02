use guardrail3_domain_report::{CheckResult, Severity};

use super::facts::LayerName;
use super::inputs::PackageLibarchInput;

const ID: &str = "RS-LIBARCH-10";

pub fn check(input: &PackageLibarchInput<'_>, results: &mut Vec<CheckResult>) {
    let package = input.package;
    if !package.layered_rules_active() {
        return;
    }

    let Some(infra) = package.layer_member(LayerName::Infra) else {
        return;
    };

    if let Some(error) = package.facade_source_error.as_deref() {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            "Root facade surface must stay readable".to_owned(),
            format!(
                "Cannot verify whether `{}` re-exports `infra`: {error}",
                package.package_rel_dir
            ),
            Some(
                package
                    .lib_rel_path
                    .clone()
                    .unwrap_or_else(|| package.cargo_rel_path.clone()),
            ),
            None,
            false,
        ));
        return;
    }
    if !package
        .facade_exports
        .iter()
        .any(|export| export.crate_name == infra.lib_crate_name)
    {
        return;
    }

    results.push(CheckResult::from_parts(
        ID.to_owned(),
        Severity::Error,
        "Infra must not become public package surface".to_owned(),
        format!(
            "Layered library `{}` re-exports `infra` crate `{}` from the root facade.",
            package.package_rel_dir, infra.lib_crate_name
        ),
        Some(
            package
                .lib_rel_path
                .clone()
                .unwrap_or_else(|| package.cargo_rel_path.clone()),
        ),
        None,
        false,
    ));
}

#[cfg(test)]
pub(super) fn run_family_check(root: &std::path::Path) -> Vec<CheckResult> {
    crate::check_test_root(root)
}

#[cfg(test)]

// reason: test-only sidecar module wiring
mod rs_libarch_10_infra_not_public_surface_tests;

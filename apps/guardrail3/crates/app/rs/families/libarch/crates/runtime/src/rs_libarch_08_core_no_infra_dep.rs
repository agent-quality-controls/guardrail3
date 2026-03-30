use guardrail3_domain_report::{CheckResult, Severity};

use super::facts::LayerName;
use super::inputs::PackageLibarchInput;

const ID: &str = "RS-LIBARCH-08";

pub fn check(input: &PackageLibarchInput<'_>, results: &mut Vec<CheckResult>) {
    let package = input.package;
    if !package.layered_rules_active() {
        return;
    }
    let Some(core) = package.layer_member(LayerName::Core) else {
        return;
    };
    let Some(infra) = package.layer_member(LayerName::Infra) else {
        return;
    };

    if let Some(error) = &core.parse_error {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            "core manifest must prove it does not depend on infra".to_owned(),
            format!(
                "Package `{}` cannot verify `core -> infra` because `{}` could not be parsed: {error}",
                package.package_rel_dir, core.cargo_rel_path
            ),
            Some(core.cargo_rel_path.clone()),
            None,
            false,
        ));
        return;
    }

    if core.direct_dependencies.contains(&infra.package_name) {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            "core must not depend on infra".to_owned(),
            format!(
                "Layer `core` in package `{}` violates `core -> infra`: it depends on package `{}`.",
                package.package_rel_dir, infra.package_name
            ),
            Some(core.cargo_rel_path.clone()),
            None,
            false,
        ));
        return;
    }

    results.push(
        CheckResult::from_parts(
            ID.to_owned(),
            Severity::Info,
            "core does not depend on infra".to_owned(),
            format!(
                "Package `{}` keeps `core` free of `infra`.",
                package.package_rel_dir
            ),
            Some(core.cargo_rel_path.clone()),
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
#[path = "rs_libarch_08_core_no_infra_dep_tests/mod.rs"] // reason: test-only sidecar module wiring
mod rs_libarch_08_core_no_infra_dep_tests;

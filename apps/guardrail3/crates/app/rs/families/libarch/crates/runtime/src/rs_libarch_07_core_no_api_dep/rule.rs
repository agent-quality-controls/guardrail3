use guardrail3_domain_report::{CheckResult, Severity};

use crate::facts::LayerName;
use crate::inputs::PackageLibarchInput;

const ID: &str = "RS-LIBARCH-07";

pub fn check(input: &PackageLibarchInput<'_>, results: &mut Vec<CheckResult>) {
    let package = input.package;
    if !package.layered_rules_active() {
        return;
    }
    let Some(core) = package.layer_member(LayerName::Core) else {
        return;
    };
    let Some(api) = package.layer_member(LayerName::Api) else {
        return;
    };

    if let Some(error) = &core.parse_error {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            "core manifest must prove it does not depend on api".to_owned(),
            format!(
                "Package `{}` cannot verify `core -> api` because `{}` could not be parsed: {error}",
                package.package_rel_dir, core.cargo_rel_path
            ),
            Some(core.cargo_rel_path.clone()),
            None,
            false,
        ));
        return;
    }

    if core.direct_dependencies.contains(&api.package_name) {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            "core must not depend on api".to_owned(),
            format!(
                "Layer `core` in package `{}` violates `core -> api`: it depends on package `{}`.",
                package.package_rel_dir, api.package_name
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
            "core does not depend on api".to_owned(),
            format!(
                "Package `{}` keeps `core` free of `api`.",
                package.package_rel_dir
            ),
            Some(core.cargo_rel_path.clone()),
            None,
            false,
        )
        .as_inventory(),
    );
}


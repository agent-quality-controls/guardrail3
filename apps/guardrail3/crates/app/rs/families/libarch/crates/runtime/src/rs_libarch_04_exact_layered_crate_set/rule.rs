use guardrail3_domain_report::{CheckResult, Severity};

use crate::facts::LayerName;
use crate::inputs::PackageLibarchInput;

const ID: &str = "RS-LIBARCH-04";

pub fn check(input: &PackageLibarchInput<'_>, results: &mut Vec<CheckResult>) {
    let package = input.package;
    if !package.layered_rules_active() {
        return;
    }

    let missing = ["api", "core"]
        .into_iter()
        .filter(|name| {
            package
                .layer_dir(LayerName::from_dir_name(name).unwrap())
                .is_none()
        })
        .collect::<Vec<_>>();
    let unexpected = package
        .layer_dirs
        .iter()
        .filter(|dir| dir.layer.is_none())
        .map(|dir| dir.name.clone())
        .collect::<Vec<_>>();

    if missing.is_empty() && unexpected.is_empty() {
        return;
    }

    results.push(CheckResult::from_parts(
        ID.to_owned(),
        Severity::Error,
        "Layered crate set must stay exact".to_owned(),
        format!(
            "Layered library `{}` must contain `api`, `core`, and optional `infra` only (missing: {:?}, extra: {:?}).",
            package.package_rel_dir, missing, unexpected
        ),
        Some(package.cargo_rel_path.clone()),
        None,
        false,
    ));
}



// reason: test-only sidecar module wiring

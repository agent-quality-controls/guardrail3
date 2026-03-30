use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::PackageLibarchInput;

const ID: &str = "RS-LIBARCH-01";

pub fn check(input: &PackageLibarchInput<'_>, results: &mut Vec<CheckResult>) {
    let package = input.package;
    if !package.is_library || package.uses_layered_mode {
        return;
    }

    if let Some(error) = package.measurement_error.as_deref() {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            "Flat library escalation must fail closed".to_owned(),
            format!(
                "Cannot verify whether `{}` may remain a flat library: {error}",
                package.package_rel_dir
            ),
            Some(package.cargo_rel_path.clone()),
            None,
            false,
        ));
        return;
    }

    if !package.escalation_required {
        return;
    }

    results.push(CheckResult::from_parts(
        ID.to_owned(),
        Severity::Error,
        "Flat library must escalate into layered mode".to_owned(),
        format!(
            "Library `{}` exceeds the flat-library thresholds ({}) and must adopt a layered workspace with `crates/api`, `crates/core`, and optional `crates/infra`.",
            package.package_rel_dir,
            package.threshold_reasons.join("; ")
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
#[path = "rs_libarch_01_escalation_required_tests/mod.rs"]
// reason: test-only sidecar module wiring
mod rs_libarch_01_escalation_required_tests;

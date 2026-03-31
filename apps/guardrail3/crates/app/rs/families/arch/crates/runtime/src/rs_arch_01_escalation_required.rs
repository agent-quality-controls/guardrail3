use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::PackageArchInput;

const ID: &str = "RS-ARCH-01";

pub fn check(input: &PackageArchInput<'_>, results: &mut Vec<CheckResult>) {
    let package = input.package;
    if !package.is_library || package.uses_split_mode {
        return;
    }

    if let Some(error) = package.measurement_error.as_deref() {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            "Flat library split requirement must fail closed".to_owned(),
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
        "Flat library must adopt split architecture".to_owned(),
        format!(
            "Library `{}` exceeds the flat-library thresholds ({}) and must adopt a split facade architecture with internal member crates.",
            package.package_rel_dir,
            package.threshold_reasons.join("; ")
        ),
        Some(package.cargo_rel_path.clone()),
        None,
        false,
    ));
}

#[cfg(test)]
#[path = "rs_arch_01_escalation_required_tests/mod.rs"]
mod rs_arch_01_escalation_required_tests;

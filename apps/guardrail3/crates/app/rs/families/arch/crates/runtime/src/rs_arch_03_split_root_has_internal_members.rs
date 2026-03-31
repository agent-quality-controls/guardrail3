use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::PackageArchInput;

const ID: &str = "RS-ARCH-03";

pub fn check(input: &PackageArchInput<'_>, results: &mut Vec<CheckResult>) {
    let package = input.package;
    if !package.split_rules_active() || !package.internal_members.is_empty() {
        return;
    }

    results.push(CheckResult::from_parts(
        ID.to_owned(),
        Severity::Error,
        "split library root must own internal member crates".to_owned(),
        format!(
            "Split library `{}` must declare one or more internal member crates beneath the root workspace.",
            package.package_rel_dir
        ),
        Some(package.cargo_rel_path.clone()),
        None,
        false,
    ));
}

#[cfg(test)]
#[path = "rs_arch_03_split_root_has_internal_members_tests/mod.rs"]
mod rs_arch_03_split_root_has_internal_members_tests;

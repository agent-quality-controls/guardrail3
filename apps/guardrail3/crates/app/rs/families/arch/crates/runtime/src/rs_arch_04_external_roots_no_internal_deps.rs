use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::PackageArchInput;

const ID: &str = "RS-ARCH-04";

pub fn check(input: &PackageArchInput<'_>, results: &mut Vec<CheckResult>) {
    let package = input.package;
    if !package.split_rules_active() {
        return;
    }

    for hit in &package.external_dependency_hits {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            "External crates must not depend on internal member crates".to_owned(),
            format!(
                "External root `{}` (package `{}`) depends directly on internal crate `{}` from split library `{}`. Depend on facade package `{}` instead.",
                hit.consumer_rel_dir,
                hit.consumer_package_name,
                hit.internal_member_package_name,
                package.package_rel_dir,
                package.package_name
            ),
            Some(hit.consumer_cargo_rel_path.clone()),
            None,
            false,
        ));
    }

    if package.external_dependency_hits.is_empty() && !package.internal_members.is_empty() {
        results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Info,
                "External roots avoid internal member crates".to_owned(),
                format!(
                    "Split library `{}` keeps external crates off its internal member packages.",
                    package.package_rel_dir
                ),
                Some(package.cargo_rel_path.clone()),
                None,
                false,
            )
            .as_inventory(),
        );
    }
}

#[cfg(test)]
#[path = "rs_arch_04_external_roots_no_internal_deps_tests/mod.rs"]
mod rs_arch_04_external_roots_no_internal_deps_tests;

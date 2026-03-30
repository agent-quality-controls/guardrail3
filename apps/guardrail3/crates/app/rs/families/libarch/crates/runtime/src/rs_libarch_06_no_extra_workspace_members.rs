use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::PackageLibarchInput;

const ID: &str = "RS-LIBARCH-06";

pub fn check(input: &PackageLibarchInput<'_>, results: &mut Vec<CheckResult>) {
    let package = input.package;
    if !package.layered_rules_active() || package.workspace_members_parse_error.is_some() {
        return;
    }

    let allowed = package.expected_layer_dir_rels();
    let extras = package
        .workspace_members
        .iter()
        .flat_map(|member| member.resolved_dirs.iter())
        .filter(|resolved| !allowed.iter().any(|allowed_dir| allowed_dir == *resolved))
        .cloned()
        .collect::<Vec<_>>();

    if extras.is_empty() {
        return;
    }

    results.push(CheckResult::from_parts(
        ID.to_owned(),
        Severity::Error,
        "Layered workspace must not include extra members".to_owned(),
        format!(
            "Layered library `{}` has workspace members outside the layered boundary: {:?}.",
            package.package_rel_dir, extras
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
#[path = "rs_libarch_06_no_extra_workspace_members_tests/mod.rs"] // reason: test-only sidecar module wiring
mod rs_libarch_06_no_extra_workspace_members_tests;

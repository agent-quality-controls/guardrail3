use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::PackageLibarchInput;

const ID: &str = "RS-LIBARCH-05";

pub fn check(input: &PackageLibarchInput<'_>, results: &mut Vec<CheckResult>) {
    let package = input.package;
    if !package.layered_rules_active() {
        return;
    }

    if let Some(error) = package.workspace_members_parse_error.as_deref() {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            "Layered workspace members must stay parseable".to_owned(),
            format!(
                "Cannot verify workspace member coverage for `{}`: {error}",
                package.package_rel_dir
            ),
            Some(package.cargo_rel_path.clone()),
            None,
            false,
        ));
        return;
    }

    let expected = package.expected_layer_dir_rels();
    let missing = expected
        .iter()
        .filter(|expected_rel| {
            !package.workspace_members.iter().any(|member| {
                member
                    .resolved_dirs
                    .iter()
                    .any(|resolved| resolved == *expected_rel)
            })
        })
        .cloned()
        .collect::<Vec<_>>();

    if missing.is_empty() {
        return;
    }

    results.push(CheckResult::from_parts(
        ID.to_owned(),
        Severity::Error,
        "Workspace members must cover layered crates".to_owned(),
        format!(
            "Layered library `{}` is missing workspace-member coverage for {:?}.",
            package.package_rel_dir, missing
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
#[path = "rs_libarch_05_workspace_members_match_layer_dirs_tests/mod.rs"]
// reason: test-only sidecar module wiring
mod rs_libarch_05_workspace_members_match_layer_dirs_tests;

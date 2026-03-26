use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::MemberManifestFailureHexarchInput;

const ID: &str = "RS-HEXARCH-26";

pub fn check(input: &MemberManifestFailureHexarchInput<'_>, results: &mut Vec<CheckResult>) {
    let failure = input.failure;
    results.push(CheckResult {
        id: ID.to_owned(),
        severity: Severity::Error,
        title: "member Cargo.toml parse error blocks hexarch dependency checks".to_owned(),
        message: format!(
            "Failed to parse `{}` for member `{}` ({}), so guardrail3 cannot verify dependency direction, inventory, purity, or cross-app boundary rules for that crate: {}",
            failure.cargo_rel_path, failure.name, failure.rel_dir, failure.parse_error
        ),
        file: Some(failure.cargo_rel_path.clone()),
        line: None,
        inventory: false,
    });
}

#[cfg(test)]
#[allow(dead_code)]
pub(super) fn results_for_test_root(root: &std::path::Path) -> Vec<CheckResult> {
    crate::check_test_tree(&test_support::walk(root))
}

#[cfg(test)]
#[allow(dead_code)]
pub(super) fn results_for_test_tree(
    tree: &guardrail3_domain_project_tree::ProjectTree,
) -> Vec<CheckResult> {
    crate::check_test_tree(tree)
}

#[cfg(test)]
#[path = "rs_hexarch_26_member_manifest_parse_error_tests/mod.rs"]
mod rs_hexarch_26_member_manifest_parse_error_tests;

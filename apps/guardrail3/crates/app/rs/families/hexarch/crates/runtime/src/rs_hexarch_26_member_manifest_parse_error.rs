use guardrail3_domain_report::{CheckResult, Severity};

use super::dependency_facts::{MemberDependencyFacts, MemberManifestFailureFacts};
use super::inputs::MemberManifestFailureHexarchInput;
use super::inventory::push_success;

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

pub fn check_inventory(
    members: &[MemberDependencyFacts],
    failures: &[MemberManifestFailureFacts],
    results: &mut Vec<CheckResult>,
) {
    if members.is_empty() || !failures.is_empty() {
        return;
    }

    push_success(
        results,
        ID,
        "member Cargo.toml files parsed cleanly".to_owned(),
        format!(
            "Hexarch parsed all {} discovered member Cargo.toml files cleanly, so dependency checks were not blocked.",
            members.len()
        ),
        None,
    );
}

#[cfg(test)]
pub(super) fn results_for_test_root(root: &std::path::Path) -> Vec<CheckResult> {
    crate::check_test_tree(&test_support::walk(root))
}
#[cfg(test)]
#[path = "rs_hexarch_26_member_manifest_parse_error_tests/mod.rs"]
mod rs_hexarch_26_member_manifest_parse_error_tests;

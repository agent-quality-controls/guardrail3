use guardrail3_domain_report::{CheckResult, Severity};

use crate::dependency_facts::{MemberDependencyFacts, MemberManifestFailureFacts};
use crate::inputs::MemberManifestFailureHexarchInput;
use crate::inventory::push_success;

const ID: &str = "RS-HEXARCH-26";

pub fn check(input: &MemberManifestFailureHexarchInput<'_>, results: &mut Vec<CheckResult>) {
    let failure = input.failure;
    results.push(CheckResult::from_parts(
    ID.to_owned(),
    Severity::Error,
    "member Cargo.toml parse error blocks hexarch dependency checks".to_owned(),
    format!(
            "Failed to parse `{}` for member `{}` ({}), so guardrail3 cannot verify dependency direction, inventory, purity, or cross-app boundary rules for that crate: {}",
            failure.cargo_rel_path, failure.name, failure.rel_dir, failure.parse_error
        ),
    Some(failure.cargo_rel_path.clone()),
    None,
    false,
    ));
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


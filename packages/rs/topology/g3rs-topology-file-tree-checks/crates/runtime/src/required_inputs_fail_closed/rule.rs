use g3rs_topology_types::G3RsTopologyFileTreeInputFailure;
use guardrail3_check_types::{G3CheckResult, G3Severity};

/// Stable identifier for this rule.
const ID: &str = "g3rs-topology/required-inputs-fail-closed";

/// Runs this rule and appends its findings to `results`.
pub(crate) fn check(input: &G3RsTopologyFileTreeInputFailure, results: &mut Vec<G3CheckResult>) {
    results.push(G3CheckResult::new(
        ID.to_owned(),
        G3Severity::Error,
        "Rust topology required input failed closed".to_owned(),
        input.message.clone(),
        Some(input.rel_path.clone()),
        None,
    ));
}

use g3rs_topology_file_tree_checks_types::G3RsTopologyFileTreeInputFailure;
use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "RS-TOPOLOGY-07";

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

#[cfg(test)]
#[path = "rs_topology_07_required_inputs_fail_closed_tests/mod.rs"]
mod tests;

use g3rs_topology_types::G3RsTopologyFileTreeInputFailure;
use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "RS-TOPOLOGY-FILETREE-07";

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
#[path = "rule_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod rule_tests;

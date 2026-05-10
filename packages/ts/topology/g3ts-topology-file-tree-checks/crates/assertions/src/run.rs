//! End-to-end assertions over the `run::check` aggregator output.
//!
//! Internal sidecar tests of `run.rs` and external integration tests share
//! these helpers so both prove the same behavior.

use g3ts_topology_file_tree_checks_runtime::check;
use g3ts_topology_types::G3TsTopologyFileTreeChecksInput;
use guardrail3_check_types::G3CheckResult;

/// Runs the file-tree checks aggregator and returns its results.
#[must_use]
pub fn run(input: &G3TsTopologyFileTreeChecksInput) -> Vec<G3CheckResult> {
    check(input)
}

/// Asserts the aggregator emitted exactly the expected set of rule ids.
///
/// # Panics
///
/// Panics when the produced rule-id set does not match `expected`.
pub fn assert_rule_ids(results: &[G3CheckResult], expected: &[&str]) {
    let actual = results.iter().map(G3CheckResult::id).collect::<Vec<_>>();
    assert_eq!(
        actual, expected,
        "expected rule ids {expected:?}, got {actual:?} from results {results:#?}"
    );
}

/// Asserts the aggregator emitted at least one finding for `rule_id`.
///
/// # Panics
///
/// Panics when no finding for `rule_id` is present in `results`.
pub fn assert_rule_fired(results: &[G3CheckResult], rule_id: &str) {
    let fired = results.iter().any(|result| result.id() == rule_id);
    assert!(
        fired,
        "expected rule `{rule_id}` to fire, got results {results:#?}"
    );
}

/// Asserts the aggregator emitted no findings for `rule_id`.
///
/// # Panics
///
/// Panics when any finding for `rule_id` is present in `results`.
pub fn assert_rule_quiet(results: &[G3CheckResult], rule_id: &str) {
    let fired = results.iter().any(|result| result.id() == rule_id);
    assert!(
        !fired,
        "expected rule `{rule_id}` to be quiet, got results {results:#?}"
    );
}

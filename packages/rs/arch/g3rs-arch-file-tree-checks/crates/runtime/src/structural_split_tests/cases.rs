use g3rs_arch_file_tree_checks_assertions::structural_split as assertions;
use g3rs_arch_types::types::G3RsArchRustPolicyState;

use super::helpers::{crate_node, run_rule, waiver};

#[test]
fn simple_crate_stays_quiet() {
    let mut node = crate_node("crate_a");
    node.has_lib_rs = true;

    let results = run_rule(&node, &G3RsArchRustPolicyState::Missing);

    assertions::assert_no_findings(&results);
}

#[test]
fn exact_thresholds_stay_quiet() {
    let mut node = crate_node("crate_a");
    node.has_lib_rs = true;
    node.max_module_depth = 3;
    node.max_sibling_dir_count = 4;
    node.max_sibling_rs_file_count = 10;

    let results = run_rule(&node, &G3RsArchRustPolicyState::Missing);

    assertions::assert_no_findings(&results);
}

#[test]
fn structural_threshold_over_limit_fires() {
    let mut node = crate_node("crate_a");
    node.has_lib_rs = true;
    node.max_module_depth = 4;
    node.max_sibling_dir_count = 5;
    node.max_sibling_rs_file_count = 11;

    let results = run_rule(&node, &G3RsArchRustPolicyState::Missing);

    assertions::assert_structural_split(&results, "crate_a/Cargo.toml");
}

#[test]
fn exact_waiver_for_crate_suppresses_structural_split() {
    let mut node = crate_node("crate_a");
    node.has_lib_rs = true;
    node.max_module_depth = 4;
    node.max_sibling_dir_count = 5;
    node.max_sibling_rs_file_count = 11;

    let results = run_rule(
        &node,
        &G3RsArchRustPolicyState::Parsed {
            rel_path: "guardrail3-rs.toml".to_owned(),
            waivers: vec![waiver(
                "g3rs-arch/structural-split",
                "crate_a/Cargo.toml",
                "structural-split",
                "Rule runtime crate intentionally aggregates one rule per file and is the package boundary by design.",
            )],
        },
    );

    assertions::assert_no_findings(&results);
}

#[test]
fn non_matching_waiver_does_not_suppress_structural_split() {
    let mut node = crate_node("crate_a");
    node.has_lib_rs = true;
    node.max_module_depth = 4;
    node.max_sibling_dir_count = 5;
    node.max_sibling_rs_file_count = 11;

    let results = run_rule(
        &node,
        &G3RsArchRustPolicyState::Parsed {
            rel_path: "guardrail3-rs.toml".to_owned(),
            waivers: vec![waiver(
                "g3rs-arch/structural-split",
                "crate_a/Cargo.toml",
                "not-structural-split",
                "Different selector should not stand down this rule.",
            )],
        },
    );

    assertions::assert_structural_split(&results, "crate_a/Cargo.toml");
}

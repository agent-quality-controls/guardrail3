use g3rs_arch_file_tree_checks_assertions::{ExpectedRuleResult, assert_rule_results, has_rule};
use guardrail3_check_types::G3Severity;

use crate::test_support::{crate_node, input};

#[test]
fn simple_crate_stays_quiet() {
    let mut node = crate_node("crate_a");
    node.has_lib_rs = true;

    let results = crate::check(&input(vec![node], Vec::new()));

    assert!(!has_rule(&results, "RS-ARCH-07"));
}

#[test]
fn exact_thresholds_stay_quiet() {
    let mut node = crate_node("crate_a");
    node.has_lib_rs = true;
    node.dependency_count = 12;
    node.max_module_depth = 3;
    node.sibling_dir_count = 4;
    node.sibling_rs_file_count = 10;

    let results = crate::check(&input(vec![node], Vec::new()));

    assert!(!has_rule(&results, "RS-ARCH-07"));
}

#[test]
fn crate_over_threshold_fires() {
    let mut node = crate_node("crate_a");
    node.has_lib_rs = true;
    node.dependency_count = 13;
    node.max_module_depth = 4;
    node.sibling_dir_count = 5;
    node.sibling_rs_file_count = 11;

    let results = crate::check(&input(vec![node], Vec::new()));

    assert_rule_results(
        &results,
        "RS-ARCH-07",
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("crate too complex, must split"),
            file: Some("crate_a/Cargo.toml"),
            inventory: Some(false),
            message: None,
        }],
    );
}

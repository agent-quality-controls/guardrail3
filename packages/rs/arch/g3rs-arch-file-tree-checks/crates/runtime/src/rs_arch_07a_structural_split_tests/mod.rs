use g3rs_arch_file_tree_checks_assertions::{ExpectedRuleResult, assert_rule_results, has_rule};
use g3rs_arch_types::G3RsArchRustPolicyState;
use guardrail3_check_types::G3Severity;

use crate::test_support::{crate_node, input, input_with_rust_policy, waiver};

#[test]
fn simple_crate_stays_quiet() {
    let mut node = crate_node("crate_a");
    node.has_lib_rs = true;

    let results = crate::check(&input(vec![node], Vec::new()));

    assert!(!has_rule(&results, "RS-ARCH-FILETREE-07"));
}

#[test]
fn exact_thresholds_stay_quiet() {
    let mut node = crate_node("crate_a");
    node.has_lib_rs = true;
    node.max_module_depth = 3;
    node.sibling_dir_count = 4;
    node.sibling_rs_file_count = 10;

    let results = crate::check(&input(vec![node], Vec::new()));

    assert!(!has_rule(&results, "RS-ARCH-FILETREE-07"));
}

#[test]
fn structural_threshold_over_limit_fires() {
    let mut node = crate_node("crate_a");
    node.has_lib_rs = true;
    node.max_module_depth = 4;
    node.sibling_dir_count = 5;
    node.sibling_rs_file_count = 11;

    let results = crate::check(&input(vec![node], Vec::new()));

    assert_rule_results(
        &results,
        "RS-ARCH-FILETREE-07",
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("crate structure too complex, must split"),
            file: Some("crate_a/Cargo.toml"),
            inventory: Some(false),
            message: None,
        }],
    );
}

#[test]
fn exact_waiver_for_crate_suppresses_structural_split() {
    let mut node = crate_node("crate_a");
    node.has_lib_rs = true;
    node.max_module_depth = 4;
    node.sibling_dir_count = 5;
    node.sibling_rs_file_count = 11;

    let results = crate::check(&input_with_rust_policy(
        vec![node],
        Vec::new(),
        G3RsArchRustPolicyState::Parsed {
            rel_path: "guardrail3-rs.toml".to_owned(),
            waivers: vec![waiver(
                "RS-ARCH-FILETREE-07",
                "crate_a/Cargo.toml",
                "structural-split",
                "Rule runtime crate intentionally aggregates one rule per file and is the package boundary by design.",
            )],
        },
    ));

    assert!(!has_rule(&results, "RS-ARCH-FILETREE-07"), "{results:#?}");
}

#[test]
fn non_matching_waiver_does_not_suppress_structural_split() {
    let mut node = crate_node("crate_a");
    node.has_lib_rs = true;
    node.max_module_depth = 4;
    node.sibling_dir_count = 5;
    node.sibling_rs_file_count = 11;

    let results = crate::check(&input_with_rust_policy(
        vec![node],
        Vec::new(),
        G3RsArchRustPolicyState::Parsed {
            rel_path: "guardrail3-rs.toml".to_owned(),
            waivers: vec![waiver(
                "RS-ARCH-FILETREE-07",
                "crate_a/Cargo.toml",
                "not-structural-split",
                "Different selector should not stand down this rule.",
            )],
        },
    ));

    assert_rule_results(
        &results,
        "RS-ARCH-FILETREE-07",
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("crate structure too complex, must split"),
            file: Some("crate_a/Cargo.toml"),
            inventory: Some(false),
            message: None,
        }],
    );
}

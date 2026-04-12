use g3rs_arch_file_tree_checks_assertions::{ExpectedRuleResult, assert_rule_results};
use guardrail3_check_types::G3Severity;

use crate::test_support::{crate_node, input};

#[test]
fn crate_with_lib_rs_inventories_facade() {
    let mut node = crate_node("crate_a");
    node.has_lib_rs = true;

    let results = crate::check(&input(vec![node], Vec::new()));

    assert_rule_results(
        &results,
        "RS-ARCH-FILETREE-01",
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Info),
            title: Some("crate has facade entry point"),
            file: Some("crate_a/Cargo.toml"),
            inventory: Some(true),
            message: None,
        }],
    );
}

#[test]
fn crate_with_main_rs_inventories_facade() {
    let mut node = crate_node("crate_a");
    node.has_main_rs = true;

    let results = crate::check(&input(vec![node], Vec::new()));

    assert_rule_results(
        &results,
        "RS-ARCH-FILETREE-01",
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Info),
            title: Some("crate has facade entry point"),
            file: Some("crate_a/Cargo.toml"),
            inventory: Some(true),
            message: Some("Crate `crate_a` has a facade entry point (main.rs)."),
        }],
    );
}

#[test]
fn crate_without_lib_or_main_fires() {
    let node = crate_node("crate_a");

    let results = crate::check(&input(vec![node], Vec::new()));

    assert_rule_results(
        &results,
        "RS-ARCH-FILETREE-01",
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("crate missing facade entry point"),
            file: Some("crate_a/Cargo.toml"),
            inventory: Some(false),
            message: None,
        }],
    );
}

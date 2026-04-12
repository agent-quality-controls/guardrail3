use g3rs_code_file_tree_checks_assertions::{ExpectedRuleResult, assert_rule_results};
use guardrail3_check_types::G3Severity;

use crate::test_support::{input, root};

#[test]
fn errors_when_root_exceeds_structural_caps() {
    let mut node = root("");
    node.max_module_depth = 7;
    node.max_sibling_dirs = 13;
    node.max_sibling_rs_files = 21;

    let results = crate::check(&input(vec![node]));

    assert_rule_results(
        &results,
        "RS-CODE-FILETREE-35",
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("crate source tree exceeds structural caps"),
            file: Some("Cargo.toml"),
            inventory: Some(false),
            message: Some(
                "Rust root `` exceeds structural caps: module depth 7 > 6, sibling source directories 13 > 12, sibling .rs files 21 > 20. Restructure the crate into smaller modules or sub-crates.",
            ),
        }],
    );
}

#[test]
fn stays_quiet_at_exact_thresholds() {
    let mut node = root("crate_a");
    node.max_module_depth = 6;
    node.max_sibling_dirs = 12;
    node.max_sibling_rs_files = 20;

    let results = crate::check(&input(vec![node]));

    assert_rule_results(&results, "RS-CODE-FILETREE-35", &[]);
}

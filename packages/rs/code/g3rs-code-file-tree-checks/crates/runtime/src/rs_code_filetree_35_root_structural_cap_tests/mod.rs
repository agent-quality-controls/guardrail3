use g3rs_code_file_tree_checks_assertions::{ExpectedRuleResult, assert_rule_results};
use guardrail3_check_types::G3Severity;

use crate::test_support::{input, root};

#[test]
fn stays_quiet_when_root_is_well_under_caps() {
    let mut node = root("crate_a");
    node.max_module_depth = 2;
    node.max_sibling_dirs = 3;
    node.max_sibling_rs_files = 4;

    let results = crate::check(&input(vec![node]));

    assert_rule_results(&results, "RS-CODE-FILETREE-35", &[]);
}

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

#[test]
fn errors_when_only_module_depth_exceeds() {
    let mut node = root("crate_a");
    node.max_module_depth = 7;
    node.max_sibling_dirs = 12;
    node.max_sibling_rs_files = 20;

    let results = crate::check(&input(vec![node]));

    assert_rule_results(
        &results,
        "RS-CODE-FILETREE-35",
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("crate source tree exceeds structural caps"),
            file: Some("crate_a/Cargo.toml"),
            inventory: Some(false),
            message: Some(
                "Rust root `crate_a` exceeds structural caps: module depth 7 > 6. Restructure the crate into smaller modules or sub-crates.",
            ),
        }],
    );
}

#[test]
fn errors_when_only_sibling_dirs_exceed() {
    let mut node = root("crate_a");
    node.max_module_depth = 6;
    node.max_sibling_dirs = 13;
    node.max_sibling_rs_files = 20;

    let results = crate::check(&input(vec![node]));

    assert_rule_results(
        &results,
        "RS-CODE-FILETREE-35",
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("crate source tree exceeds structural caps"),
            file: Some("crate_a/Cargo.toml"),
            inventory: Some(false),
            message: Some(
                "Rust root `crate_a` exceeds structural caps: sibling source directories 13 > 12. Restructure the crate into smaller modules or sub-crates.",
            ),
        }],
    );
}

#[test]
fn errors_when_only_sibling_rs_files_exceed() {
    let mut node = root("crate_a");
    node.max_module_depth = 6;
    node.max_sibling_dirs = 12;
    node.max_sibling_rs_files = 21;

    let results = crate::check(&input(vec![node]));

    assert_rule_results(
        &results,
        "RS-CODE-FILETREE-35",
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("crate source tree exceeds structural caps"),
            file: Some("crate_a/Cargo.toml"),
            inventory: Some(false),
            message: Some(
                "Rust root `crate_a` exceeds structural caps: sibling .rs files 21 > 20. Restructure the crate into smaller modules or sub-crates.",
            ),
        }],
    );
}

#[test]
fn errors_when_depth_and_dirs_exceed() {
    let mut node = root("crate_a");
    node.max_module_depth = 7;
    node.max_sibling_dirs = 13;
    node.max_sibling_rs_files = 20;

    let results = crate::check(&input(vec![node]));

    assert_rule_results(
        &results,
        "RS-CODE-FILETREE-35",
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("crate source tree exceeds structural caps"),
            file: Some("crate_a/Cargo.toml"),
            inventory: Some(false),
            message: Some(
                "Rust root `crate_a` exceeds structural caps: module depth 7 > 6, sibling source directories 13 > 12. Restructure the crate into smaller modules or sub-crates.",
            ),
        }],
    );
}

#[test]
fn errors_when_depth_and_rs_files_exceed() {
    let mut node = root("crate_a");
    node.max_module_depth = 7;
    node.max_sibling_dirs = 12;
    node.max_sibling_rs_files = 21;

    let results = crate::check(&input(vec![node]));

    assert_rule_results(
        &results,
        "RS-CODE-FILETREE-35",
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("crate source tree exceeds structural caps"),
            file: Some("crate_a/Cargo.toml"),
            inventory: Some(false),
            message: Some(
                "Rust root `crate_a` exceeds structural caps: module depth 7 > 6, sibling .rs files 21 > 20. Restructure the crate into smaller modules or sub-crates.",
            ),
        }],
    );
}

#[test]
fn errors_when_dirs_and_rs_files_exceed() {
    let mut node = root("crate_a");
    node.max_module_depth = 6;
    node.max_sibling_dirs = 13;
    node.max_sibling_rs_files = 21;

    let results = crate::check(&input(vec![node]));

    assert_rule_results(
        &results,
        "RS-CODE-FILETREE-35",
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("crate source tree exceeds structural caps"),
            file: Some("crate_a/Cargo.toml"),
            inventory: Some(false),
            message: Some(
                "Rust root `crate_a` exceeds structural caps: sibling source directories 13 > 12, sibling .rs files 21 > 20. Restructure the crate into smaller modules or sub-crates.",
            ),
        }],
    );
}

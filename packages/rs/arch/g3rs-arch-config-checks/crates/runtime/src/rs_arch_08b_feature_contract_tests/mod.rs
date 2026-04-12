use g3rs_arch_config_checks_assertions::{ExpectedRuleResult, assert_rule_results, has_rule};
use guardrail3_check_types::G3Severity;

use crate::test_support::{config_crate, input};

#[test]
fn crates_without_public_exports_do_not_require_feature_contract() {
    let node = config_crate("crate_a");

    let results = crate::check(&input(vec![node], Vec::new()));

    assert!(!has_rule(&results, "RS-ARCH-08B"));
}

#[test]
fn missing_all_feature_fires() {
    let mut node = config_crate("crate_a");
    node.requires_feature_contract = true;

    let results = crate::check(&input(vec![node], Vec::new()));

    assert_rule_results(
        &results,
        "RS-ARCH-08B",
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("missing `all` feature"),
            file: Some("crate_a/Cargo.toml"),
            inventory: Some(false),
            message: None,
        }],
    );
}

#[test]
fn valid_feature_contract_inventories() {
    let mut node = config_crate("crate_a");
    node.requires_feature_contract = true;
    node.has_all_feature = true;
    node.all_feature_deps = vec!["api".to_owned()];
    node.has_default_feature = true;
    node.default_feature_deps = vec!["all".to_owned()];

    let results = crate::check(&input(vec![node], Vec::new()));

    assert_rule_results(
        &results,
        "RS-ARCH-08B",
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Info),
            title: Some("feature contract supports facade exports"),
            file: Some("crate_a/Cargo.toml"),
            inventory: Some(true),
            message: None,
        }],
    );
}

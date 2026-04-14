use g3rs_arch_config_checks_assertions::{ExpectedRuleResult, assert_rule_results, has_rule};
use guardrail3_check_types::G3Severity;

use crate::test_support::{config_crate, input};

#[test]
fn crates_without_public_exports_do_not_require_feature_contract() {
    let node = config_crate("crate_a");

    let results = crate::check(&input(vec![node], Vec::new()));

    assert!(!has_rule(&results, "RS-ARCH-CONFIG-08"));
}

#[test]
fn missing_default_feature_fires_when_feature_contract_has_no_default() {
    let mut node = config_crate("crate_a");
    node.requires_feature_contract = true;
    node.has_default_feature = false;

    let results = crate::check(&input(vec![node], Vec::new()));

    assert_rule_results(
        &results,
        "RS-ARCH-CONFIG-08",
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("missing `default` feature"),
            file: Some("crate_a/Cargo.toml"),
            inventory: Some(false),
            message: None,
        }],
    );
}

#[test]
fn empty_default_feature_fires_when_feature_contract_enables_nothing() {
    let mut node = config_crate("crate_a");
    node.requires_feature_contract = true;
    node.has_default_feature = true;
    node.default_feature_deps = Vec::new();

    let results = crate::check(&input(vec![node], Vec::new()));

    assert_rule_results(
        &results,
        "RS-ARCH-CONFIG-08",
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("`default` feature is empty"),
            file: Some("crate_a/Cargo.toml"),
            inventory: Some(false),
            message: None,
        }],
    );
}

#[test]
fn valid_named_feature_contract_inventories_without_all() {
    let mut node = config_crate("crate_a");
    node.requires_feature_contract = true;
    node.has_default_feature = true;
    node.default_feature_deps = vec!["api".to_owned()];

    let results = crate::check(&input(vec![node], Vec::new()));

    assert_rule_results(
        &results,
        "RS-ARCH-CONFIG-08",
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Info),
            title: Some("feature contract supports facade exports"),
            file: Some("crate_a/Cargo.toml"),
            inventory: Some(true),
            message: None,
        }],
    );
}

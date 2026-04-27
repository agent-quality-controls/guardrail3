use g3rs_arch_config_checks_assertions::feature_contract as assertions;

use super::helpers::{config_crate, run_rule};

#[test]
fn crates_without_public_exports_do_not_require_feature_contract() {
    let results = run_rule(&config_crate("crate_a"));

    assertions::assert_no_findings(&results);
}

#[test]
fn missing_default_feature_fires_when_feature_contract_has_no_default() {
    let mut node = config_crate("crate_a");
    node.requires_feature_contract = true;

    let results = run_rule(&node);

    assertions::assert_missing_default_feature(&results, "crate_a/Cargo.toml");
}

#[test]
fn empty_default_feature_fires_when_feature_contract_enables_nothing() {
    let mut node = config_crate("crate_a");
    node.requires_feature_contract = true;
    node.has_default_feature = true;

    let results = run_rule(&node);

    assertions::assert_empty_default_feature(&results, "crate_a/Cargo.toml");
}

#[test]
fn valid_named_feature_contract_inventories_without_all() {
    let mut node = config_crate("crate_a");
    node.requires_feature_contract = true;
    node.has_default_feature = true;
    node.default_feature_deps = vec!["api".to_owned()];

    let results = run_rule(&node);

    assertions::assert_feature_inventory(&results, "crate_a/Cargo.toml");
}

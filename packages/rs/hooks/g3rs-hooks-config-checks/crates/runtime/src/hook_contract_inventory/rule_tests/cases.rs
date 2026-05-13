use g3rs_hooks_config_checks_assertions::hook_contract_inventory::rule as assertions;

use super::super::check;
use super::helpers::{hook, requirement};

#[test]
fn emits_inventory_with_family_owned_rule_id() {
    let mut results = Vec::new();

    check(&hook(), &[requirement()], &mut results);

    assertions::assert_contract_loaded(
        &results,
        "g3rs-fmt/hook-contract",
        "fmt",
        "fmt hook contract is loaded with 1 trigger pattern(s), 1 required command(s), and 1 critical command(s).",
    );
}

#[test]
fn emits_no_inventory_when_no_contracts_are_loaded() {
    let mut results = Vec::new();

    check(&hook(), &[], &mut results);

    assertions::assert_no_contract_inventory(&results, "g3rs-fmt/hook-contract");
}

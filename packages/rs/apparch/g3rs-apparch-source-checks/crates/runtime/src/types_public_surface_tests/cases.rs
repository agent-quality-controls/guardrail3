use g3rs_apparch_source_checks_assertions::types_public_surface as assertions;

use super::helpers::{free_function_input, inherent_method_input, run_rule, trait_only_input};

#[test]
fn public_free_function_in_types_fires() {
    let results = run_rule(&free_function_input());

    assertions::assert_behavior_violation(&results, "types/contracts/src/lib.rs");
}

#[test]
fn public_inherent_method_in_types_fires() {
    let results = run_rule(&inherent_method_input());

    assertions::assert_behavior_violation(&results, "types/contracts/src/order.rs");
}

#[test]
fn trait_only_types_crate_emits_inventory() {
    let results = run_rule(&trait_only_input());

    assertions::assert_clean_inventory(&results, "types/contracts/Cargo.toml");
}

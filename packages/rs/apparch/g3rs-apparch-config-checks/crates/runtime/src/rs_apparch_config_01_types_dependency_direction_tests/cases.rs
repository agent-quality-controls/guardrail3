use g3rs_apparch_config_checks_assertions::rs_apparch_config_01_types_dependency_direction as assertions;

use super::helpers::{input, run_rule};

#[test]
fn forbidden_logic_dependency_fires() {
    let results = run_rule(&input(&[("types/core/Cargo.toml", "logic/service/Cargo.toml")]));

    assertions::assert_forbidden_dependency(&results, "types/core/Cargo.toml");
}

#[test]
fn forbidden_same_layer_dependency_fires() {
    let results = run_rule(&input(&[("types/core/Cargo.toml", "types/shared/Cargo.toml")]));

    assertions::assert_forbidden_dependency(&results, "types/core/Cargo.toml");
}

#[test]
fn clean_types_crate_emits_inventory() {
    let results = run_rule(&input(&[]));

    assertions::assert_clean_inventory(&results, "types/core/Cargo.toml");
}

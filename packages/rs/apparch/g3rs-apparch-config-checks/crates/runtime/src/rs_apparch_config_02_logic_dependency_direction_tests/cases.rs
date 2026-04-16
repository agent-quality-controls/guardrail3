use g3rs_apparch_config_checks_assertions::rs_apparch_config_02_logic_dependency_direction as assertions;

use super::helpers::{input, run_rule};

#[test]
fn forbidden_io_dependency_fires() {
    let results = run_rule(&input(&[("logic/service/Cargo.toml", "io/outbound/db/Cargo.toml")]));

    assertions::assert_forbidden_dependency(&results, "logic/service/Cargo.toml");
}

#[test]
fn forbidden_io_inbound_dependency_fires() {
    let results = run_rule(&input(&[("logic/service/Cargo.toml", "io/inbound/http/Cargo.toml")]));

    assertions::assert_forbidden_dependency(&results, "logic/service/Cargo.toml");
}

#[test]
fn forbidden_same_layer_dependency_fires() {
    let results = run_rule(&input(&[("logic/service/Cargo.toml", "logic/shared/Cargo.toml")]));

    assertions::assert_forbidden_dependency(&results, "logic/service/Cargo.toml");
}

#[test]
fn logic_depends_on_types_stays_allowed() {
    let results = run_rule(&input(&[("logic/service/Cargo.toml", "types/core/Cargo.toml")]));

    assertions::assert_clean_inventory(&results, "logic/service/Cargo.toml");
}

#[test]
fn clean_logic_crate_emits_inventory() {
    let results = run_rule(&input(&[]));

    assertions::assert_clean_inventory(&results, "logic/service/Cargo.toml");
}

use g3rs_apparch_config_checks_assertions::logic_dependency_direction as assertions;

use super::helpers::{input, run_rule};

#[test]
fn forbidden_io_dependency_fires() {
    let results = run_rule(
        &input(&[("logic/service/Cargo.toml", "io/outbound/db/Cargo.toml")]),
        "logic/service/Cargo.toml",
    );

    assertions::assert_forbidden_dependency(&results, "logic/service/Cargo.toml", "db");
}

#[test]
fn forbidden_io_inbound_dependency_fires() {
    let results = run_rule(
        &input(&[("logic/service/Cargo.toml", "io/inbound/http/Cargo.toml")]),
        "logic/service/Cargo.toml",
    );

    assertions::assert_forbidden_dependency(&results, "logic/service/Cargo.toml", "http");
}

#[test]
fn forbidden_same_layer_dependency_fires() {
    let results = run_rule(
        &input(&[("logic/service/Cargo.toml", "logic/shared/Cargo.toml")]),
        "logic/service/Cargo.toml",
    );

    assertions::assert_forbidden_dependency(&results, "logic/service/Cargo.toml", "shared");
}

#[test]
fn logic_depends_on_types_stays_allowed() {
    let results = run_rule(
        &input(&[("logic/service/Cargo.toml", "types/core/Cargo.toml")]),
        "logic/service/Cargo.toml",
    );

    assertions::assert_clean_inventory(&results, "logic/service/Cargo.toml", "service");
}

#[test]
fn clean_logic_crate_emits_inventory() {
    let results = run_rule(&input(&[]), "logic/service/Cargo.toml");

    assertions::assert_clean_inventory(&results, "logic/service/Cargo.toml", "service");
}

#[test]
fn package_internal_assertions_dependency_stays_allowed() {
    let results = run_rule(
        &input(&[(
            "logic/validate-command/crates/assertions/Cargo.toml",
            "logic/validate-command/crates/runtime/Cargo.toml",
        )]),
        "logic/validate-command/crates/assertions/Cargo.toml",
    );

    assertions::assert_clean_inventory(
        &results,
        "logic/validate-command/crates/assertions/Cargo.toml",
        "validate-command-assertions",
    );
}

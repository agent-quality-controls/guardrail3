use g3rs_apparch_config_checks_assertions::rs_apparch_config_03_io_outbound_dependency_direction as assertions;

use super::helpers::{input, run_rule};

#[test]
fn forbidden_logic_dependency_fires() {
    let results = run_rule(&input(&[(
        "io/outbound/db/Cargo.toml",
        "logic/service/Cargo.toml",
    )]), "io/outbound/db/Cargo.toml");

    assertions::assert_forbidden_dependency(&results, "io/outbound/db/Cargo.toml");
}

#[test]
fn forbidden_io_inbound_dependency_fires() {
    let results = run_rule(&input(&[(
        "io/outbound/db/Cargo.toml",
        "io/inbound/http/Cargo.toml",
    )]), "io/outbound/db/Cargo.toml");

    assertions::assert_forbidden_dependency(&results, "io/outbound/db/Cargo.toml");
}

#[test]
fn forbidden_same_layer_dependency_fires() {
    let results = run_rule(&input(&[(
        "io/outbound/db/Cargo.toml",
        "io/outbound/cache/Cargo.toml",
    )]), "io/outbound/db/Cargo.toml");

    assertions::assert_forbidden_dependency(&results, "io/outbound/db/Cargo.toml");
}

#[test]
fn clean_outbound_crate_emits_inventory() {
    let results = run_rule(&input(&[(
        "io/outbound/db/Cargo.toml",
        "types/core/Cargo.toml",
    )]), "io/outbound/db/Cargo.toml");

    assertions::assert_clean_inventory(&results, "io/outbound/db/Cargo.toml");
}

#[test]
fn package_internal_assertions_dependency_stays_allowed() {
    let results = run_rule(&input(&[(
        "io/outbound/report/crates/assertions/Cargo.toml",
        "io/outbound/report/crates/runtime/Cargo.toml",
    )]), "io/outbound/report/crates/assertions/Cargo.toml");

    assertions::assert_clean_inventory(
        &results,
        "io/outbound/report/crates/assertions/Cargo.toml",
    );
}

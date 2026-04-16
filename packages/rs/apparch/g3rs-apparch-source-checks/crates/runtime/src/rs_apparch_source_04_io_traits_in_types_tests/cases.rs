use g3rs_apparch_source_checks_assertions::rs_apparch_source_04_io_traits_in_types as assertions;
use g3rs_apparch_types::G3RsApparchLayer;

use super::helpers::{
    clean_io_input, io_trait_input, logic_trait_input, run_rule,
};

#[test]
fn io_public_trait_fires() {
    let results = run_rule(&io_trait_input(
        G3RsApparchLayer::IoOutbound,
        "io/outbound/db/Cargo.toml",
        "io/outbound/db/src/lib.rs",
        "DbPort",
    ));

    assertions::assert_trait_violation(&results, "io/outbound/db/src/lib.rs");
}

#[test]
fn io_inbound_public_trait_fires() {
    let results = run_rule(&io_trait_input(
        G3RsApparchLayer::IoInbound,
        "io/inbound/http/Cargo.toml",
        "io/inbound/http/src/lib.rs",
        "InboundPort",
    ));

    assertions::assert_trait_violation(&results, "io/inbound/http/src/lib.rs");
}

#[test]
fn logic_public_trait_stays_quiet() {
    let results = run_rule(&logic_trait_input());

    assertions::assert_no_findings(&results);
}

#[test]
fn clean_io_crate_emits_inventory() {
    let results = run_rule(&clean_io_input(
        G3RsApparchLayer::IoInbound,
        "io/inbound/http/Cargo.toml",
    ));

    assertions::assert_clean_inventory(&results, "io/inbound/http/Cargo.toml");
}

#[test]
fn clean_outbound_io_crate_emits_inventory() {
    let results = run_rule(&clean_io_input(
        G3RsApparchLayer::IoOutbound,
        "io/outbound/db/Cargo.toml",
    ));

    assertions::assert_clean_inventory(&results, "io/outbound/db/Cargo.toml");
}

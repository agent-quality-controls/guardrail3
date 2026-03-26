use super::super::{run_source_case, SourceCrateLayerForTest};
use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_22_ports_trait_dominance as assertions;

#[test]
fn impl_heavy_ports_warns() {
    let results = run_source_case(
        SourceCrateLayerForTest::Ports,
        "api-ports-http",
        "apps/api/crates/ports/http",
        1,
        3,
        None,
        None,
    );

    assertions::assert_warning_count(&results, "", 1);
}

#[test]
fn equal_impl_and_public_trait_counts_do_not_warn() {
    let results = run_source_case(
        SourceCrateLayerForTest::Ports,
        "api-ports-http",
        "apps/api/crates/ports/http",
        2,
        2,
        None,
        None,
    );

    assertions::assert_no_warning(&results, "");
}

#[test]
fn dto_only_ports_crate_stays_clean() {
    let results = run_source_case(
        SourceCrateLayerForTest::Ports,
        "api-ports-http",
        "apps/api/crates/ports/http",
        0,
        0,
        None,
        None,
    );

    assertions::assert_no_warning(&results, "");
}

#[test]
fn non_ports_crates_are_ignored() {
    let results = run_source_case(
        SourceCrateLayerForTest::Adapters,
        "api-adapters-http",
        "apps/api/crates/adapters/http",
        0,
        99,
        None,
        None,
    );

    assertions::assert_no_warning(&results, "");
}

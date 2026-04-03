use super::helpers::{SourceCrateLayerForTest, run_source_case};
use guardrail3_app_rs_family_hexarch_assertions::dependency_integrity::rs_hexarch_22_ports_trait_dominance as assertions;

#[test]
fn dto_only_ports_crate_stays_clean() {
    let results = run_source_case(
        SourceCrateLayerForTest::Ports,
        "api-ports-http",
        "apps/api/crates/ports/http",
        0,
        0,
        0,
        None,
        None,
    );

    assertions::assert_no_warning(&results, "");
}

#[test]
fn public_free_functions_warn() {
    let results = run_source_case(
        SourceCrateLayerForTest::Ports,
        "api-ports-http",
        "apps/api/crates/ports/http",
        1,
        2,
        0,
        None,
        None,
    );

    assertions::assert_warning_count(&results, "", 1);
    assertions::assert_warning_title_contains(&results, "", &["exposes public free functions"]);
}

#[test]
fn public_inherent_methods_warn() {
    let results = run_source_case(
        SourceCrateLayerForTest::Ports,
        "api-ports-http",
        "apps/api/crates/ports/http",
        1,
        0,
        3,
        None,
        None,
    );

    assertions::assert_warning_count(&results, "", 1);
    assertions::assert_warning_title_contains(&results, "", &["exposes public inherent methods"]);
}

#[test]
fn non_ports_crates_are_ignored() {
    let results = run_source_case(
        SourceCrateLayerForTest::Adapters,
        "api-adapters-http",
        "apps/api/crates/adapters/http",
        0,
        99,
        99,
        None,
        None,
    );

    assertions::assert_no_warning(&results, "");
}

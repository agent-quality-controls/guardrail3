use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_15_boundary_config as assertions;
use super::copy_fixture;

#[test]
fn golden_fixture_has_no_boundary_config_hits() {
    let tmp = copy_fixture();
    let results = super::run_family(tmp.path());
    let hits = assertions::error_results(&results, "");

    assert!(
        hits.is_empty(),
        "the golden fixture already has boundary configs for every app boundary: {hits:#?}"
    );
}

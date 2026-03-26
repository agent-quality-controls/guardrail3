use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_15_boundary_config as assertions;
use crate::test_support::copy_fixture;

#[test]
fn golden_fixture_has_no_boundary_config_hits() {
    let tmp = copy_fixture();
    let results = assertions::run_family(tmp.path());
    let hits: Vec<_> = results
        .iter()
        .filter(|result| result.id == "RS-HEXARCH-15")
        .collect();

    assert!(
        hits.is_empty(),
        "the golden fixture already has boundary configs for every app boundary: {hits:#?}"
    );
}

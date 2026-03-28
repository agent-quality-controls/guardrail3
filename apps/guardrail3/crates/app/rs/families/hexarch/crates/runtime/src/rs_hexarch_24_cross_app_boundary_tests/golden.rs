use super::copy_fixture;
use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_24_cross_app_boundary as assertions;

#[test]
fn golden_fixture_has_no_cross_app_boundary_errors() {
    let tmp = copy_fixture();
    let results = super::run_family(tmp.path());
    assertions::assert_no_error(&results, "");
}

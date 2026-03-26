use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_15_boundary_config as assertions;
use super::copy_fixture;

#[test]
fn golden_fixture_has_no_boundary_config_hits() {
    let tmp = copy_fixture();
    let results = super::run_family(tmp.path());
    assertions::assert_no_error(&results, "");
}

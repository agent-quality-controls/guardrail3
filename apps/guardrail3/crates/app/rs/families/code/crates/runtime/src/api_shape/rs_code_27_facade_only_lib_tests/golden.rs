use super::super::copy_fixture;
use super::super::run_family;
use guardrail3_app_rs_family_code_assertions::api_shape::rs_code_27_facade_only_lib::assert_no_hits;

#[test]
fn populated_golden_fixture_has_no_facade_only_lib_hits() {
    let fixture = copy_fixture();

    let results = run_family(fixture.path());
    assert_no_hits(&results);
}

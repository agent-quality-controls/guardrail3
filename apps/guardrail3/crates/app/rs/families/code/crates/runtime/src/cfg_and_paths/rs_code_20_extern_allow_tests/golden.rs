use super::super::copy_fixture;
use super::super::run_family;
use guardrail3_app_rs_family_code_assertions::rs_code_20_extern_allow::assert_no_hits;

#[test]
fn populated_golden_fixture_has_no_extern_allow_hits() {
    let fixture = copy_fixture();

    let results = run_family(fixture.path());
    assert_no_hits(&results);
}

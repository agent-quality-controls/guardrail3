use super::super::copy_fixture;
use super::super::run_family;
use guardrail3_app_rs_family_code_assertions::rs_code_04_item_level_allow_with_reason::assert_no_hits;

#[test]
fn populated_golden_fixture_has_no_documented_item_allow_hits() {
    let fixture = copy_fixture();

    let results = run_family(fixture.path());

    assert_no_hits(&results);
}

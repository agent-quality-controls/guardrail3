use super::helpers::copy_fixture;
use super::helpers::run_family;
use guardrail3_app_rs_family_code_assertions::hygiene::rs_code_11_use_count_warn::assert_no_hits;

#[test]
fn populated_golden_fixture_has_no_use_count_warn_hits() {
    let fixture = copy_fixture();

    let results = run_family(fixture.path());
    assert_no_hits(&results);
}

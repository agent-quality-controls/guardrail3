use super::super::copy_fixture;
use super::super::run_family;
use guardrail3_app_rs_family_code_assertions::inventory::rs_code_29_large_trait_inventory::assert_no_hits;

#[test]
fn populated_golden_fixture_has_no_large_trait_hits() {
    let fixture = copy_fixture();

    let results = run_family(fixture.path());
    assert_no_hits(&results);
}

use guardrail3_app_rs_family_code_assertions::rs_code_19_large_type_inventory::{assert_no_hits};
use super::super::run_family;
use super::super::copy_fixture;

#[test]
fn populated_golden_fixture_has_no_large_type_inventory_hits() {
    let fixture = copy_fixture();

    let results = run_family(fixture.path());
    assert_no_hits(&results);
}

use super::super::copy_fixture;
use super::super::run_family;
use guardrail3_app_rs_family_code_assertions::lint_policy::rs_code_07_exception_comment_inventory::assert_no_hits;

#[test]
fn populated_golden_fixture_has_no_exception_comment_inventory_hits() {
    let fixture = copy_fixture();

    let results = run_family(fixture.path());
    assert_no_hits(&results);
}

use super::super::copy_fixture;
use super::super::run_family;
use guardrail3_app_rs_family_code_assertions::hygiene::rs_code_13_todo_macros::assert_no_hits;

#[test]
fn populated_golden_fixture_has_no_todo_macro_hits() {
    let fixture = copy_fixture();

    let results = run_family(fixture.path());
    assert_no_hits(&results);
}

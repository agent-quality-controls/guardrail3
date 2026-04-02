use super::super::copy_fixture;
use super::super::run_family;
use guardrail3_app_rs_family_code_assertions::cfg_and_paths::rs_code_18_always_true_cfg_attr_bypass::assert_no_hits;

#[test]
fn populated_golden_fixture_has_no_always_true_cfg_attr_hits() {
    let fixture = copy_fixture();

    let results = run_family(fixture.path());
    assert_no_hits(&results);
}

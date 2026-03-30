use super::super::copy_fixture;
use super::super::run_family;
use guardrail3_app_rs_family_code_assertions::rs_code_17_impl_allow_blast_radius::assert_no_hits;

#[test]
fn populated_golden_fixture_has_no_impl_allow_blast_radius_hits() {
    let fixture = copy_fixture();

    let results = run_family(fixture.path());
    assert_no_hits(&results);
}

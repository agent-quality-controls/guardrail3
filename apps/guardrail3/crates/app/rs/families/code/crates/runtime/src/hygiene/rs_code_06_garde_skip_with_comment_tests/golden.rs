use super::super::copy_fixture;
use super::super::run_family;
use guardrail3_app_rs_family_code_assertions::hygiene::rs_code_06_garde_skip_with_comment::assert_no_hits;

#[test]
fn populated_golden_fixture_has_no_plain_comment_garde_skip_hits() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let results = run_family(root);

    assert_no_hits(&results);
}

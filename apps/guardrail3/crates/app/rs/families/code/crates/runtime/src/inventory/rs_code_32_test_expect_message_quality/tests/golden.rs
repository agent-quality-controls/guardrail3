use guardrail3_app_rs_family_code_assertions::inventory::rs_code_32_test_expect_message_quality::assert_no_hits;

use super::helpers::copy_fixture;
use super::helpers::run_family;

#[test]
fn populated_golden_fixture_has_no_weak_test_expect_messages() {
    let fixture = copy_fixture();
    let results = run_family(fixture.path());

    assert_no_hits(&results);
}

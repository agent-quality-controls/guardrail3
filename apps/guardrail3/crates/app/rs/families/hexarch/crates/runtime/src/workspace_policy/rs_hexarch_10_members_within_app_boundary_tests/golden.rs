use super::copy_fixture;
use guardrail3_app_rs_family_hexarch_assertions::workspace_policy::rs_hexarch_10_members_within_app_boundary as assertions;

#[test]
fn golden_fixture_has_no_rule_10_errors() {
    let tmp = copy_fixture();
    let results = super::run_family(tmp.path());
    assertions::assert_no_error(&results, "");
}

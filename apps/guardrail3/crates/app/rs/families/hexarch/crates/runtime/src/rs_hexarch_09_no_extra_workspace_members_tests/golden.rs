use super::copy_fixture;
use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_09_no_extra_workspace_members as assertions;

#[test]
fn golden_fixture_has_no_rule_09_errors() {
    let tmp = copy_fixture();
    let results = super::run_family(tmp.path());
    assertions::assert_no_error(&results, "");
}

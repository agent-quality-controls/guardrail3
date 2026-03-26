use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_07_workspace_members_match_crate_dirs as assertions;
use super::copy_fixture;

#[test]
fn golden_fixture_has_no_rule_07_errors() {
    let tmp = copy_fixture();
    let results = super::run_family(tmp.path());
    assertions::assert_no_error(&results, "");
}

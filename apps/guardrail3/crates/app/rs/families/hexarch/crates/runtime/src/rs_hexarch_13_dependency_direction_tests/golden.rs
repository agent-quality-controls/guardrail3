use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_13_dependency_direction as assertions;
use super::copy_fixture;

#[test]
fn golden_fixture_has_no_rule_13_errors() {
    let tmp = copy_fixture();

    let results = super::run_family(tmp.path());
    assertions::assert_no_error(&results, "");
}

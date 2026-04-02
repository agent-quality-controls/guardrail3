use super::copy_fixture;
use guardrail3_app_rs_family_hexarch_assertions::structure::rs_hexarch_06_leaf_valid as assertions;

#[test]
fn golden_has_no_rule_06_errors() {
    let tmp = copy_fixture();
    let results = super::run_family(tmp.path());
    assertions::assert_no_error(&results, "");
}

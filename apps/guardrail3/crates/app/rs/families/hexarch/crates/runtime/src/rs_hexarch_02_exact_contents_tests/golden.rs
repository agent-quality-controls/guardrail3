use super::copy_fixture;
use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_02_exact_contents as assertions;

#[test]
fn golden_has_no_rule_02_errors() {
    let tmp = copy_fixture();
    let results = super::run_family(tmp.path());
    assertions::assert_no_error(&results, "");
}

use super::copy_fixture;
use guardrail3_app_rs_family_hexarch_assertions::structure::rs_hexarch_04_loose_files as assertions;

#[test]
fn golden_has_no_rule_04_errors() {
    let tmp = copy_fixture();
    let results = super::run_family(tmp.path());
    assertions::assert_no_error(&results, "");
}

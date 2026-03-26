use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_19_same_layer_cycles as assertions;
use super::copy_fixture;

#[test]
fn golden_fixture_has_no_rule_19_errors() {
    let tmp = copy_fixture();

    let results = super::run_family(tmp.path());
    assertions::assert_no_error(&results, "");
}

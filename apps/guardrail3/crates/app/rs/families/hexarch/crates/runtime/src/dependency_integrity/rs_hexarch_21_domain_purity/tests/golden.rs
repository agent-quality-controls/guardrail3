use super::copy_fixture;
use guardrail3_app_rs_family_hexarch_assertions::dependency_integrity::rs_hexarch_21_domain_purity as assertions;

#[test]
fn golden_fixture_has_no_rule_21_errors() {
    let tmp = copy_fixture();

    let results = super::run_family(tmp.path());
    assertions::assert_no_error(&results, "");
}

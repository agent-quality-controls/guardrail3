use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_02_exact_contents as assertions;
use test_support::copy_fixture;

#[test]
fn golden_has_no_rule_02_errors() {
    let tmp = copy_fixture();
    let results = assertions::run_family(tmp.path());
    assert!(
        assertions::errors_by_id(&results, "RS-HEXARCH-02").is_empty(),
        "{results:#?}"
    );
}

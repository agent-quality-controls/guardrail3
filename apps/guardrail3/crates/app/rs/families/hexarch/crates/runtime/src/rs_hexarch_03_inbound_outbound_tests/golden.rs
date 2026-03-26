use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_03_inbound_outbound as assertions;
use test_support::copy_fixture;

#[test]
fn golden_has_no_rule_03_errors() {
    let tmp = copy_fixture();
    let results = assertions::run_family(tmp.path());
    assert!(
        assertions::errors_by_id(&results, "RS-HEXARCH-03").is_empty(),
        "{results:#?}"
    );
}

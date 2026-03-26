use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_09_no_extra_workspace_members as assertions;
use test_support::copy_fixture;

#[test]
fn golden_fixture_has_no_rule_09_errors() {
    let tmp = copy_fixture();
    let results = assertions::run_family(tmp.path());
    assert!(
        assertions::errors_by_id(&results, "RS-HEXARCH-09").is_empty(),
        "{results:#?}"
    );
}

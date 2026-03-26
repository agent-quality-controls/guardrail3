use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_20_dev_dependency_direction as assertions;
use crate::test_support::copy_fixture;

#[test]
fn golden_fixture_has_no_dev_direction_errors() {
    let tmp = copy_fixture();
    let results = assertions::run_family(tmp.path());
    assert!(
        assertions::errors_by_id(&results, "RS-HEXARCH-20").is_empty(),
        "{results:#?}"
    );
}

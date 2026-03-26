use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_23_adapter_pub_trait as assertions;
use test_support::copy_fixture;

#[test]
fn golden_fixture_has_no_adapter_public_trait_errors() {
    let tmp = copy_fixture();
    let results = assertions::run_family(tmp.path());
    assert!(
        assertions::errors_by_id(&results, "RS-HEXARCH-23").is_empty(),
        "{results:#?}"
    );
}

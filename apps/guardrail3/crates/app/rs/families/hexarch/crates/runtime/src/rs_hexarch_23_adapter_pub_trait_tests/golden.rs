use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_23_adapter_pub_trait as assertions;
use super::copy_fixture;

#[test]
fn golden_fixture_has_no_adapter_public_trait_errors() {
    let tmp = copy_fixture();
    let results = super::run_family(tmp.path());
    assertions::assert_no_error(&results, "");
}

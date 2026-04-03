use super::helpers::check_source;
use guardrail3_app_rs_family_code_assertions::api_shape::rs_code_34_generic_parameter_cap::assert_no_hits;

#[test]
fn ignores_lifetimes_and_allows_six_type_const_parameters() {
    assert_no_hits(&check_source(
        "src/lib.rs",
        "pub fn build<'a, A, B, C, D, E, F>(_value: &'a str) {}",
    ));
    assert_no_hits(&check_source(
        "src/lib.rs",
        "pub trait Service<A, B, C, D, E, const N: usize> {}",
    ));
}

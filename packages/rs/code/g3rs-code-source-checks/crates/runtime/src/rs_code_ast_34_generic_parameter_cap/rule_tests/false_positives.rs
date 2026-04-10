use super::helpers::check_source;
use g3rs_code_source_checks_assertions::rs_code_34_generic_parameter_cap::assert_rule_results;

#[test]
fn ignores_lifetimes_and_allows_six_type_const_parameters() {
    assert_rule_results(
        &check_source(
            "src/lib.rs",
            "pub fn build<'a, A, B, C, D, E, F>(_value: &'a str) {}",
            false,
        ),
        &[],
    );
    assert_rule_results(
        &check_source(
            "src/lib.rs",
            "pub trait Service<A, B, C, D, E, const N: usize> {}",
            false,
        ),
        &[],
    );
}

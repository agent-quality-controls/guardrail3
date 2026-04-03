use super::helpers::check_source;
use guardrail3_app_rs_family_code_assertions::api_shape::rs_code_34_generic_parameter_cap::{
    RuleFinding, assert_findings,
};

#[test]
fn errors_on_functions_and_structs_above_generic_cap() {
    let results = check_source(
        "src/lib.rs",
        "pub fn build<A, B, C, D, E, F, G>() {}\npub struct Cache<A, B, C, D, E, F, const N: usize>;",
    );

    assert_findings(
        &results,
        &[
            RuleFinding::new(
                guardrail3_domain_report::Severity::Error,
                "too many generic parameters",
                "function `build` has 7 type/const generic parameters (cap 6; lifetimes do not count).",
                Some("src/lib.rs"),
                Some(1),
                false,
            ),
            RuleFinding::new(
                guardrail3_domain_report::Severity::Error,
                "too many generic parameters",
                "struct `Cache` has 7 type/const generic parameters (cap 6; lifetimes do not count).",
                Some("src/lib.rs"),
                Some(2),
                false,
            ),
        ],
    );
}

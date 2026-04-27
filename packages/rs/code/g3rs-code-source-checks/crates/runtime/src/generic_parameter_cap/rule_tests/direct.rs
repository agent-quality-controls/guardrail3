use g3rs_code_source_checks_assertions::generic_parameter_cap::rule::{
    ExpectedRuleResult, G3Severity, assert_rule_results,
};

#[test]
fn errors_on_functions_and_structs_above_generic_cap() {
    let results = super::super::check_source(
        "src/lib.rs",
        "pub fn build<A, B, C, D, E, F, G>() {}\npub struct Cache<A, B, C, D, E, F, const N: usize>;",
        false,
    );

    assert_rule_results(
        &results,
        &[
            ExpectedRuleResult {
                severity: Some(G3Severity::Error),
                title: Some("too many generic parameters"),
                file: Some("src/lib.rs"),
                inventory: Some(false),
                message: Some(
                    "function `build` has 7 type/const generic parameters (cap 6; lifetimes do not count). Reduce the number of generic parameters or introduce a trait to abstract them.",
                ),
                line: Some(1),
            },
            ExpectedRuleResult {
                severity: Some(G3Severity::Error),
                title: Some("too many generic parameters"),
                file: Some("src/lib.rs"),
                inventory: Some(false),
                message: Some(
                    "struct `Cache` has 7 type/const generic parameters (cap 6; lifetimes do not count). Reduce the number of generic parameters or introduce a trait to abstract them.",
                ),
                line: Some(2),
            },
        ],
    );
}

#[test]
fn errors_on_traits_above_generic_cap() {
    let results = super::super::check_source(
        "src/lib.rs",
        "pub trait Service<A, B, C, D, E, F, const N: usize> {}",
        false,
    );

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("too many generic parameters"),
            file: Some("src/lib.rs"),
            inventory: Some(false),
            message: Some(
                "trait `Service` has 7 type/const generic parameters (cap 6; lifetimes do not count). Reduce the number of generic parameters or introduce a trait to abstract them.",
            ),
            line: Some(1),
        }],
    );
}

#[test]
fn errors_on_enums_above_generic_cap() {
    let results = super::super::check_source(
        "src/lib.rs",
        "pub enum Response<A, B, C, D, E, F, const N: usize> { Ok, Err }",
        false,
    );

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("too many generic parameters"),
            file: Some("src/lib.rs"),
            inventory: Some(false),
            message: Some(
                "enum `Response` has 7 type/const generic parameters (cap 6; lifetimes do not count). Reduce the number of generic parameters or introduce a trait to abstract them.",
            ),
            line: Some(1),
        }],
    );
}

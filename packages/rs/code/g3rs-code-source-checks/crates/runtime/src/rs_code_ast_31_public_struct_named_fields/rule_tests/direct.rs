use g3rs_code_source_checks_assertions::rs_code_ast_31_public_struct_named_fields::rule::{
    ExpectedRuleResult, G3Severity, assert_rule_results,
};

#[test]
fn warns_on_public_struct_with_two_public_fields() {
    let content = "pub struct User { pub id: String, pub email: String }";
    let results = super::super::check_source("src/lib.rs", content, false);

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Warn),
            title: Some("public struct exposes named public fields"),
            file: Some("src/lib.rs"),
            inventory: Some(false),
            message: Some(
                "Struct `User` exposes 2 named `pub` fields. Make the fields private and expose constructors or getters instead, so callers use one API instead of reaching into raw state.",
            ),
            line: Some(1),
        }],
    );
}

#[test]
fn errors_on_public_struct_with_five_public_fields() {
    let content = "pub struct User { pub a: u8, pub b: u8, pub c: u8, pub d: u8, pub e: u8 }";
    let results = super::super::check_source("src/lib.rs", content, false);

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("public struct exposes named public fields"),
            file: Some("src/lib.rs"),
            inventory: Some(false),
            message: Some(
                "Struct `User` exposes 5 named `pub` fields. Make the fields private and expose constructors or getters instead, so callers use one API instead of reaching into raw state.",
            ),
            line: Some(1),
        }],
    );
}

#[test]
fn warns_on_public_struct_with_four_public_fields() {
    let content = "pub struct User { pub a: u8, pub b: u8, pub c: u8, pub d: u8 }";
    let results = super::super::check_source("src/lib.rs", content, false);

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Warn),
            title: Some("public struct exposes named public fields"),
            file: Some("src/lib.rs"),
            inventory: Some(false),
            message: Some(
                "Struct `User` exposes 4 named `pub` fields. Make the fields private and expose constructors or getters instead, so callers use one API instead of reaching into raw state.",
            ),
            line: Some(1),
        }],
    );
}

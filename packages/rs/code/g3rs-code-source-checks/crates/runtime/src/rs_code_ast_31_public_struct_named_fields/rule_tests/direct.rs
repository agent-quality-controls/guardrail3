use super::helpers::check_source;
use g3rs_code_source_checks_assertions::rs_code_31_public_struct_named_fields::{
    ExpectedRuleResult, G3Severity, assert_rule_results,
};

#[test]
fn warns_on_public_struct_with_two_public_fields() {
    let content = "pub struct User { pub id: String, pub email: String }";
    let results = check_source("src/lib.rs", content);

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Warn),
            title: Some("public struct exposes named public fields"),
            file: Some("src/lib.rs"),
            inventory: Some(false),
            message: Some(
                "Public struct `User` exposes 2 named `pub` fields (warn below 5, error at 5+). Prefer private fields and explicit accessors or constructors.",
            ),
            line: Some(1),
        }],
    );
}

#[test]
fn errors_on_public_struct_with_five_public_fields() {
    let content = "pub struct User { pub a: u8, pub b: u8, pub c: u8, pub d: u8, pub e: u8 }";
    let results = check_source("src/lib.rs", content);

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("public struct exposes named public fields"),
            file: Some("src/lib.rs"),
            inventory: Some(false),
            message: Some(
                "Public struct `User` exposes 5 named `pub` fields (warn below 5, error at 5+). Prefer private fields and explicit accessors or constructors.",
            ),
            line: Some(1),
        }],
    );
}

#[test]
fn warns_on_public_struct_with_four_public_fields() {
    let content = "pub struct User { pub a: u8, pub b: u8, pub c: u8, pub d: u8 }";
    let results = check_source("src/lib.rs", content);

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Warn),
            title: Some("public struct exposes named public fields"),
            file: Some("src/lib.rs"),
            inventory: Some(false),
            message: Some(
                "Public struct `User` exposes 4 named `pub` fields (warn below 5, error at 5+). Prefer private fields and explicit accessors or constructors.",
            ),
            line: Some(1),
        }],
    );
}

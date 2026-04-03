use super::helpers::check_source;
use guardrail3_app_rs_family_code_assertions::api_shape::rs_code_31_public_struct_named_fields::{
    RuleFinding, assert_findings,
};

#[test]
fn errors_on_public_struct_with_named_public_fields() {
    let content = "pub struct User { pub id: String, pub email: String }";
    let results = check_source("src/lib.rs", content);

    assert_findings(
        &results,
        &[RuleFinding::new(
            guardrail3_domain_report::Severity::Error,
            "public struct exposes named public fields",
            "Public struct `User` exposes 2 named `pub` fields. Prefer private fields and explicit accessors or constructors.",
            Some("src/lib.rs"),
            Some(1),
            false,
        )],
    );
}

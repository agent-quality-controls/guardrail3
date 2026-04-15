use super::helpers::check_source_with_shared;
use g3rs_code_source_checks_assertions::rs_code_31_public_struct_named_fields::{
    ExpectedRuleResult, G3Severity, assert_rule_results,
};

#[test]
fn shared_plain_record_struct_is_allowed() {
    let results = check_source_with_shared(
        "src/types.rs",
        "pub struct Input { pub rel_path: String, pub profile: Option<String> }",
        true,
    );

    assert_rule_results(&results, &[]);
}

#[test]
fn shared_struct_with_inherent_impl_still_errors() {
    let results = check_source_with_shared(
        "src/types.rs",
        "pub struct Input { pub rel_path: String, pub profile: Option<String>, pub raw: String, pub flags: usize, pub mode: bool }\nimpl Input { pub fn validate(&self) -> bool { self.mode } }",
        true,
    );

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("public struct exposes named public fields"),
            file: Some("src/types.rs"),
            inventory: Some(false),
            message: Some(
                "Shared-crate struct `Input` exposes 5 named `pub` fields and also defines inherent methods. Keep shared public fields only on plain data structs. Make the fields private or move the behavior out, so shared crates stay as transport data instead of mixing data and API logic.",
            ),
            line: Some(1),
        }],
    );
}

#[test]
fn shared_struct_with_mixed_visibility_still_warns() {
    let results = check_source_with_shared(
        "src/types.rs",
        "pub struct Input { pub rel_path: String, profile: Option<String> }",
        true,
    );

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Warn),
            title: Some("public struct exposes named public fields"),
            file: Some("src/types.rs"),
            inventory: Some(false),
            message: Some(
                "Shared-crate struct `Input` exposes 1 named `pub` fields but also hides some fields. In shared crates, either make this a plain data struct with all fields `pub`, or make the fields private and expose an API. Mixed visibility hides part of the shared data contract.",
            ),
            line: Some(1),
        }],
    );
}

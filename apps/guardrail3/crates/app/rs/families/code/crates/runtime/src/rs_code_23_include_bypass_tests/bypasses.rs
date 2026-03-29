use super::super::check_source;
use guardrail3_app_rs_family_code_assertions::rs_code_23_include_bypass::{
    RuleFinding, assert_findings,
};

#[test]
fn warns_on_out_dir_concat_with_nested_parent_segment() {
    let content = "include!(concat!(env!(\"OUT_DIR\"), \"/generated/../escape.rs\"));";
    let results = check_source("src/lib.rs", content, false);

    assert_findings(
        &results,
        &[RuleFinding {
            severity: guardrail3_domain_report::Severity::Warn,
            title: "include path traversal",
            message: "`include!()` build-script pattern appends a path containing `..`.",
            file: Some("src/lib.rs"),
            line: Some(1),
            inventory: false,
        }],
    );
}

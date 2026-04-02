use super::super::check_source;
use guardrail3_app_rs_family_code_assertions::cfg_and_paths::rs_code_23_include_bypass::{
    RuleFinding, assert_findings,
};

#[test]
fn warns_on_out_dir_concat_with_nested_parent_segment() {
    let content = "include!(concat!(env!(\"OUT_DIR\"), \"/generated/../escape.rs\"));";
    let results = check_source("src/lib.rs", content, false);

    assert_findings(
        &results,
        &[RuleFinding::new(
            guardrail3_domain_report::Severity::Warn,
            "include path traversal",
            "`include!()` build-script pattern appends a path containing `..`.",
            Some("src/lib.rs"),
            Some(1),
            false,
        )],
    );
}

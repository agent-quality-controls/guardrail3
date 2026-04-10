use super::helpers::check_source;
use g3rs_code_source_checks_assertions::rs_code_23_include_bypass::{
    ExpectedRuleResult, G3Severity, assert_rule_results,
};

#[test]
fn reports_plain_include_bypass() {
    let results = check_source("src/lib.rs", "include!(\"../generated.rs\");", false);

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("include! bypass"),
            file: Some("src/lib.rs"),
            inventory: Some(false),
            message: Some("`include!()` pulls in Rust code outside the scanned file boundary."),
            line: Some(1),
        }],
    );
}

#[test]
fn inventories_build_script_include_pattern() {
    let content = "include!(concat!(env!(\"OUT_DIR\"), \"/generated.rs\"));";
    let results = check_source("src/lib.rs", content, false);

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Warn),
            title: Some("build-script include! inventory"),
            file: Some("src/lib.rs"),
            inventory: Some(false),
            message: Some(
                "`include!(concat!(env!(\"OUT_DIR\"), ...))` detected. Review generated-code boundary.",
            ),
            line: Some(1),
        }],
    );
}

#[test]
fn warns_on_include_path_traversal_cases() {
    let build_script = "include!(concat!(env!(\"OUT_DIR\"), \"/../escape.rs\"));";
    let bytes = "const BYTES: &[u8] = include_bytes!(\"../fixtures/payload.bin\");";

    assert_rule_results(
        &check_source("src/lib.rs", build_script, false),
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Warn),
            title: Some("include path traversal"),
            file: Some("src/lib.rs"),
            inventory: Some(false),
            message: Some("`include!()` build-script pattern appends a path containing `..`."),
            line: Some(1),
        }],
    );

    assert_rule_results(
        &check_source("src/lib.rs", bytes, false),
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Warn),
            title: Some("include path traversal"),
            file: Some("src/lib.rs"),
            inventory: Some(false),
            message: Some("`include_bytes!()` uses a path containing `..`."),
            line: Some(1),
        }],
    );
}

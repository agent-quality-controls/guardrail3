use super::helpers::check_source;
use g3rs_code_ast_checks_assertions::rs_code_33_public_weak_error_forms::{
    ExpectedRuleResult, G3Severity, assert_rule_results,
};

#[test]
fn errors_on_string_and_str_public_result_errors() {
    let results = check_source(
        "src/lib.rs",
        "pub fn parse() -> Result<(), String> { Ok(()) }\npub fn label() -> Result<(), &str> { Ok(()) }",
        false,
    );

    assert_rule_results(
        &results,
        &[
            ExpectedRuleResult {
                severity: Some(G3Severity::Error),
                title: Some("weak public error form"),
                file: Some("src/lib.rs"),
                inventory: Some(false),
                message: Some(
                    "Public function `parse` returns `Result<_, String>`. Use a typed public error instead.",
                ),
                line: Some(1),
            },
            ExpectedRuleResult {
                severity: Some(G3Severity::Error),
                title: Some("weak public error form"),
                file: Some("src/lib.rs"),
                inventory: Some(false),
                message: Some(
                    "Public function `label` returns `Result<_, &str>`. Use a typed public error instead.",
                ),
                line: Some(2),
            },
        ],
    );
}

#[test]
fn errors_on_anyhow_and_box_dyn_error_public_results() {
    let results = check_source(
        "src/lib.rs",
        "pub fn parse() -> Result<(), anyhow::Error> { Ok(()) }\npub fn boxed() -> Result<(), Box<dyn std::error::Error>> { Ok(()) }",
        false,
    );

    assert_rule_results(
        &results,
        &[
            ExpectedRuleResult {
                severity: Some(G3Severity::Error),
                title: Some("weak public error form"),
                file: Some("src/lib.rs"),
                inventory: Some(false),
                message: Some(
                    "Public function `parse` returns `Result<_, anyhow::Error>`. Use a typed public error instead.",
                ),
                line: Some(1),
            },
            ExpectedRuleResult {
                severity: Some(G3Severity::Error),
                title: Some("weak public error form"),
                file: Some("src/lib.rs"),
                inventory: Some(false),
                message: Some(
                    "Public function `boxed` returns `Result<_, Box<dyn Error>>`. Use a typed public error instead.",
                ),
                line: Some(2),
            },
        ],
    );
}

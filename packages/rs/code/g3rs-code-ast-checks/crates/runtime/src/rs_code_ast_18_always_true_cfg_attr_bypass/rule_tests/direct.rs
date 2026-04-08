use super::helpers::check_source;
use g3rs_code_ast_checks_assertions::rs_code_18_always_true_cfg_attr_bypass::{
    ExpectedRuleResult, G3Severity, assert_rule_results,
};

#[test]
fn errors_on_unconditional_cfg_attr_bypass() {
    let content = r#"
#[cfg_attr(not(any()), allow(clippy::unwrap_used))]
fn foo() {}
"#;
    let results = check_source("src/foo.rs", content, false);

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("always-true cfg_attr bypass"),
            file: Some("src/foo.rs"),
            inventory: Some(false),
            message: Some(
                "`#[cfg_attr(..., allow(clippy::unwrap_used))]` is effectively unconditional. Use a direct `#[allow]` with an explicit reason instead.",
            ),
            line: Some(2),
        }],
    );
}

#[test]
fn errors_on_trait_item_with_always_true_cfg_attr_allow() {
    let content = "trait Api {\n    #[cfg_attr(all(), allow(dead_code))]\n    fn run();\n}\n";
    let results = check_source("src/lib.rs", content, false);

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("always-true cfg_attr bypass"),
            file: Some("src/lib.rs"),
            inventory: Some(false),
            message: Some(
                "`#[cfg_attr(..., allow(dead_code))]` is effectively unconditional. Use a direct `#[allow]` with an explicit reason instead.",
            ),
            line: Some(2),
        }],
    );
}

use g3rs_code_source_checks_assertions::test_expect_message_quality::rule::{
    ExpectedRuleResult, G3Severity, assert_rule_results,
};

#[test]
fn errors_on_short_expect_message_in_test_file() {
    let results = super::super::check_source(
        "tests/probe.rs",
        "fn probe() { let _ = Some(1).expect(\"ok\"); }",
        true,
    );

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("test expect message too weak"),
            file: Some("tests/probe.rs"),
            inventory: Some(false),
            message: Some(
                "Test `expect(...)` message must explain the failure clearly. Weak message `ok` found in `fn probe() { let _ = Some(1).expect(\"ok\"); }`.",
            ),
            line: Some(1),
        }],
    );
}

#[test]
fn errors_on_non_literal_expect_message_in_test_file() {
    let content =
        "fn probe() { let msg = \"backend fixture should parse\"; let _ = Some(1).expect(msg); }";
    let results = super::super::check_source("tests/probe.rs", content, true);

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("test expect message must be literal"),
            file: Some("tests/probe.rs"),
            inventory: Some(false),
            message: Some(
                "Test `expect(...)` message must be a useful string literal or `concat!` of string literals, not an indirect expression: `fn probe() { let msg = \"backend fixture should parse\"; let _ = Some(1).expect(msg); }`.",
            ),
            line: Some(1),
        }],
    );
}

#[test]
fn errors_on_cfg_test_expect_inside_non_test_file() {
    let content = "#[cfg(test)]\nmod tests {\n    #[test]\n    fn probe() { let _ = Some(1).expect(\"present\"); }\n}";
    let results = super::super::check_source("src/lib.rs", content, false);

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("test expect message too weak"),
            file: Some("src/lib.rs"),
            inventory: Some(false),
            message: Some(
                "Test `expect(...)` message must explain the failure clearly. Weak message `present` found in `fn probe() { let _ = Some(1).expect(\"present\"); }`.",
            ),
            line: Some(4),
        }],
    );
}

use super::super::check_source;
use guardrail3_app_rs_family_code_assertions::inventory::rs_code_32_test_expect_message_quality::{
    RuleFinding, assert_findings,
};

#[test]
fn errors_on_short_expect_message_in_test_file() {
    let results = check_source(
        "tests/probe.rs",
        "fn probe() { let _ = Some(1).expect(\"ok\"); }",
        true,
    );

    assert_findings(
        &results,
        &[RuleFinding::new(
            guardrail3_domain_report::Severity::Error,
            "test expect message too weak",
            "Test `expect(...)` message must explain the failure clearly. Weak message `ok` found in `fn probe() { let _ = Some(1).expect(\"ok\"); }`.",
            Some("tests/probe.rs"),
            Some(1),
            false,
        )],
    );
}

#[test]
fn errors_on_non_literal_expect_message_in_test_file() {
    let content =
        "fn probe() { let msg = \"backend fixture should parse\"; let _ = Some(1).expect(msg); }";
    let results = check_source("tests/probe.rs", content, true);

    assert_findings(
        &results,
        &[RuleFinding::new(
            guardrail3_domain_report::Severity::Error,
            "test expect message must be literal",
            "Test `expect(...)` message must be a useful string literal or `concat!` of string literals, not an indirect expression: `fn probe() { let msg = \"backend fixture should parse\"; let _ = Some(1).expect(msg); }`.",
            Some("tests/probe.rs"),
            Some(1),
            false,
        )],
    );
}

#[test]
fn errors_on_cfg_test_expect_inside_non_test_file() {
    let content = "#[cfg(test)]\nmod tests {\n    #[test]\n    fn probe() { let _ = Some(1).expect(\"present\"); }\n}";
    let results = check_source("src/lib.rs", content, false);

    assert_findings(
        &results,
        &[RuleFinding::new(
            guardrail3_domain_report::Severity::Error,
            "test expect message too weak",
            "Test `expect(...)` message must explain the failure clearly. Weak message `present` found in `fn probe() { let _ = Some(1).expect(\"present\"); }`.",
            Some("src/lib.rs"),
            Some(4),
            false,
        )],
    );
}

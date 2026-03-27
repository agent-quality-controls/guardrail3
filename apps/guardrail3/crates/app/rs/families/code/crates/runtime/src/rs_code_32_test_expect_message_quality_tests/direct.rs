use guardrail3_domain_report::Severity;

use guardrail3_app_rs_family_code_assertions::rs_code_32_test_expect_message_quality::{
    assert_normalized_len, findings,
};
use super::super::check_source;

#[test]
fn errors_on_short_expect_message_in_test_file() {
    let content = "fn probe() { let _ = Some(1).expect(\"ok\"); }";
    let binding = check_source("tests/probe.rs", content, true);
    let results = findings(&binding);

    assert_normalized_len(&results, 1);
    assert_eq!(results[0].id, "RS-CODE-32");
    assert_eq!(results[0].severity, Severity::Error);
    assert_eq!(results[0].title, "test expect message too weak");
    assert_eq!(results[0].line, Some(1));
    assert_eq!(results[0].file.as_deref(), Some("tests/probe.rs"));
}

#[test]
fn errors_on_non_literal_expect_message_in_test_file() {
    let content = "fn probe() { let msg = \"backend fixture should parse\"; let _ = Some(1).expect(msg); }";
    let binding = check_source("tests/probe.rs", content, true);
    let results = findings(&binding);

    assert_normalized_len(&results, 1);
    assert_eq!(results[0].id, "RS-CODE-32");
    assert_eq!(results[0].severity, Severity::Error);
    assert_eq!(results[0].title, "test expect message must be literal");
}

#[test]
fn errors_on_cfg_test_expect_inside_non_test_file() {
    let content = "#[cfg(test)]\nmod tests {\n    #[test]\n    fn probe() { let _ = Some(1).expect(\"present\"); }\n}";
    let binding = check_source("src/lib.rs", content, false);
    let results = findings(&binding);

    assert_normalized_len(&results, 1);
    assert_eq!(results[0].id, "RS-CODE-32");
    assert_eq!(results[0].severity, Severity::Error);
    assert_eq!(results[0].title, "test expect message too weak");
    assert_eq!(results[0].line, Some(4));
}

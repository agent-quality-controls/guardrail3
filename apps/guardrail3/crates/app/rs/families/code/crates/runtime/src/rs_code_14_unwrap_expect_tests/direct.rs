use guardrail3_domain_report::Severity;

use guardrail3_app_rs_family_code_assertions::rs_code_14_unwrap_expect::{assert_normalized_len, findings};
use super::super::check_source;

#[test]
fn warns_on_unwrap_usage() {
    let content = "fn foo() { let _ = some_option().unwrap(); }";
    let raw_results = check_source("src/foo.rs", content, false);
    let results = findings(&raw_results);

    assert_normalized_len(&results, 1);
    assert_eq!(results[0].id, "RS-CODE-14");
    assert_eq!(results[0].severity, Severity::Warn);
    assert_eq!(results[0].file.as_deref(), Some("src/foo.rs"));
    assert_eq!(results[0].line, Some(1));
    assert_eq!(results[0].title, ".unwrap() usage");
    assert_eq!(
        results[0].message,
        "`.unwrap()` found: fn foo() { let _ = some_option().unwrap(); }."
    );
}

#[test]
fn warns_on_expect_usage() {
    let content = "fn foo() { let _ = some_option().expect(\"present\"); }";
    let raw_results = check_source("src/foo.rs", content, false);
    let results = findings(&raw_results);

    assert_normalized_len(&results, 1);
    assert_eq!(results[0].id, "RS-CODE-14");
    assert_eq!(results[0].severity, Severity::Warn);
    assert_eq!(results[0].file.as_deref(), Some("src/foo.rs"));
    assert_eq!(results[0].line, Some(1));
    assert_eq!(results[0].title, ".expect() usage");
    assert_eq!(
        results[0].message,
        "`.expect()` found: fn foo() { let _ = some_option().expect(\"present\"); }."
    );
}

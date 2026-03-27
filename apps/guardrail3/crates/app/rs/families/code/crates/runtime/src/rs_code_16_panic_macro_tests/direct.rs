use guardrail3_domain_report::Severity;

use guardrail3_app_rs_family_code_assertions::rs_code_16_panic_macro::{assert_normalized_len, findings};
use super::super::check_source;

#[test]
fn warns_on_panic_macro_in_non_test_code() {
    let content = "fn foo() { panic!(\"boom\"); }";
    let raw_results = check_source("src/foo.rs", content, false);
    let results = findings(&raw_results);

    assert_normalized_len(&results, 1);
    assert_eq!(results[0].id, "RS-CODE-16");
    assert_eq!(results[0].severity, Severity::Warn);
    assert_eq!(results[0].file.as_deref(), Some("src/foo.rs"));
    assert_eq!(results[0].line, Some(1));
    assert_eq!(results[0].title, "panic! macro");
    assert_eq!(
        results[0].message,
        "`panic!()` macro found: fn foo() { panic!(\"boom\"); }."
    );
}

#[test]
fn warns_on_qualified_panic_macro_in_non_test_code() {
    let content = "fn foo() { core::panic!(\"boom\"); }";
    let raw_results = check_source("src/foo.rs", content, false);
    let results = findings(&raw_results);

    assert_normalized_len(&results, 1);
    assert_eq!(results[0].id, "RS-CODE-16");
    assert_eq!(results[0].severity, Severity::Warn);
    assert_eq!(results[0].file.as_deref(), Some("src/foo.rs"));
    assert_eq!(results[0].line, Some(1));
    assert_eq!(results[0].title, "panic! macro");
    assert_eq!(
        results[0].message,
        "`panic!()` macro found: fn foo() { core::panic!(\"boom\"); }."
    );
}

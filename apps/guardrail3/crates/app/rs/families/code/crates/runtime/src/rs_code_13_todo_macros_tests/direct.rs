use guardrail3_domain_report::Severity;

use super::super::check_source;
use guardrail3_app_rs_family_code_assertions::rs_code_13_todo_macros::{
    assert_normalized_len, findings,
};

#[test]
fn warns_on_todo_macro() {
    let content = "fn foo() { todo!(); }";
    let raw_results = check_source("src/foo.rs", content, false);
    let results = findings(&raw_results);

    assert_normalized_len(&results, 1);
    assert_eq!(results[0].id, "RS-CODE-13");
    assert_eq!(results[0].severity, Severity::Warn);
    assert_eq!(results[0].file.as_deref(), Some("src/foo.rs"));
    assert_eq!(results[0].line, Some(1));
    assert_eq!(results[0].title, "todo! macro");
    assert_eq!(
        results[0].message,
        "`todo!()` macro found: fn foo() { todo!(); }."
    );
}

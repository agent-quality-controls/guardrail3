use guardrail3_domain_report::Severity;

use guardrail3_app_rs_family_code_assertions::rs_code_28_inline_pub_mod_in_lib::{assert_normalized_len, findings};
use super::super::check_source;

#[test]
fn warns_on_inline_public_module_in_lib_rs() {
    let content = "pub mod api { pub fn run() {} }";
    let binding = check_source("src/lib.rs", content, false);
    let results = findings(&binding);

    assert_normalized_len(&results, 1);
    assert_eq!(results[0].id, "RS-CODE-28");
    assert_eq!(results[0].severity, Severity::Warn);
    assert_eq!(results[0].title, "inline public module in lib.rs");
    assert_eq!(
        results[0].message,
        "`pub mod api { ... }` should live in its own file."
    );
    assert_eq!(results[0].line, Some(1));
    assert_eq!(results[0].file.as_deref(), Some("src/lib.rs"));
    assert!(!results[0].inventory);
}

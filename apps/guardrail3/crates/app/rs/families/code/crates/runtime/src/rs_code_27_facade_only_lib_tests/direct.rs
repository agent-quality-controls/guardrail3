use guardrail3_domain_report::Severity;

use guardrail3_app_rs_family_code_assertions::rs_code_27_facade_only_lib::{assert_normalized_len, findings};
use super::super::check_source;

#[test]
fn errors_on_private_use_in_library_lib_rs() {
    let content = "use crate::internal::Thing;";
    let binding = check_source("src/lib.rs", content, false);
    let results = findings(&binding);

    assert_normalized_len(&results, 1);
    assert_eq!(results[0].id, "RS-CODE-27");
    assert_eq!(results[0].severity, Severity::Error);
    assert_eq!(results[0].title, "lib.rs should stay facade-only");
    assert_eq!(
        results[0].message,
        "lib.rs contains private use `crate::internal::Thing`. Keep lib.rs limited to facade declarations and type/const definitions."
    );
    assert_eq!(results[0].line, Some(1));
    assert_eq!(results[0].file.as_deref(), Some("src/lib.rs"));
    assert!(!results[0].inventory);
}

#[test]
fn errors_on_inline_public_module_in_library_lib_rs() {
    let content = "pub mod api { pub fn run() {} }";
    let binding = check_source("src/lib.rs", content, false);
    let results = findings(&binding);

    assert_normalized_len(&results, 1);
    assert_eq!(results[0].id, "RS-CODE-27");
    assert_eq!(results[0].severity, Severity::Error);
    assert_eq!(results[0].title, "lib.rs should stay facade-only");
    assert_eq!(
        results[0].message,
        "lib.rs contains inline module `api`. Keep lib.rs limited to facade declarations and type/const definitions."
    );
    assert_eq!(results[0].line, Some(1));
    assert_eq!(results[0].file.as_deref(), Some("src/lib.rs"));
    assert!(!results[0].inventory);
}

#[test]
fn errors_on_cfg_test_inline_module_in_library_lib_rs() {
    let content = "#[cfg(test)]\npub mod tests { pub fn run() {} }";
    let binding = check_source("src/lib.rs", content, false);
    let results = findings(&binding);

    assert_normalized_len(&results, 1);
    assert_eq!(results[0].id, "RS-CODE-27");
    assert_eq!(results[0].severity, Severity::Error);
    assert_eq!(results[0].title, "lib.rs should stay facade-only");
    assert_eq!(
        results[0].message,
        "lib.rs contains inline module `tests`. Keep lib.rs limited to facade declarations and type/const definitions."
    );
    assert_eq!(results[0].line, Some(2));
    assert_eq!(results[0].file.as_deref(), Some("src/lib.rs"));
    assert!(!results[0].inventory);
}

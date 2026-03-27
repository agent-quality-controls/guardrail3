use guardrail3_domain_report::Severity;

use guardrail3_app_rs_family_code_assertions::rs_code_23_include_bypass::{assert_normalized_len, findings};
use super::super::check_source;

#[test]
fn errors_on_plain_include_bypass() {
    let content = "include!(\"../generated.rs\");";
    let binding = check_source("src/lib.rs", content, false);
    let results = findings(&binding);

    assert_normalized_len(&results, 1);
    assert_eq!(results[0].id, "RS-CODE-23");
    assert_eq!(results[0].severity, Severity::Error);
    assert_eq!(results[0].title, "include! bypass");
    assert_eq!(
        results[0].message,
        "`include!()` pulls in Rust code outside the scanned file boundary."
    );
    assert_eq!(results[0].line, Some(1));
    assert_eq!(results[0].file.as_deref(), Some("src/lib.rs"));
    assert!(!results[0].inventory);
}

#[test]
fn inventories_build_script_include_pattern() {
    let content = "include!(concat!(env!(\"OUT_DIR\"), \"/generated.rs\"));";
    let binding = check_source("src/lib.rs", content, false);
    let results = findings(&binding);

    assert_normalized_len(&results, 1);
    assert_eq!(results[0].id, "RS-CODE-23");
    assert_eq!(results[0].severity, Severity::Info);
    assert_eq!(results[0].title, "build-script include! inventory");
    assert_eq!(
        results[0].message,
        "`include!(concat!(env!(\"OUT_DIR\"), ...))` detected. Review generated-code boundary."
    );
    assert_eq!(results[0].line, Some(1));
    assert_eq!(results[0].file.as_deref(), Some("src/lib.rs"));
    assert!(results[0].inventory);
}

#[test]
fn warns_on_include_path_traversal() {
    let content = "const BYTES: &[u8] = include_bytes!(\"../fixtures/payload.bin\");";
    let binding = check_source("src/lib.rs", content, false);
    let results = findings(&binding);

    assert_normalized_len(&results, 1);
    assert_eq!(results[0].id, "RS-CODE-23");
    assert_eq!(results[0].severity, Severity::Warn);
    assert_eq!(results[0].title, "include path traversal");
    assert_eq!(
        results[0].message,
        "`include_bytes!()` uses a path containing `..`."
    );
    assert_eq!(results[0].line, Some(1));
    assert_eq!(results[0].file.as_deref(), Some("src/lib.rs"));
    assert!(!results[0].inventory);
}

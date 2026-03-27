use guardrail3_domain_report::Severity;

use guardrail3_app_rs_family_code_assertions::rs_code_15_direct_fs_usage::{assert_normalized_len, findings};
use super::super::check_source;

#[test]
fn errors_on_std_fs_import() {
    let content = "use std::fs;\nfn main() {}";
    let raw_results = check_source("src/foo.rs", content, false);
    let results = findings(&raw_results);

    assert_normalized_len(&results, 1);
    assert_eq!(results[0].id, "RS-CODE-15");
    assert_eq!(results[0].severity, Severity::Error);
    assert_eq!(results[0].file.as_deref(), Some("src/foo.rs"));
    assert_eq!(results[0].line, Some(1));
    assert_eq!(results[0].title, "direct std::fs import");
    assert_eq!(
        results[0].message,
        "Direct `use std::fs` import found: `use std::fs;`."
    );
}

#[test]
fn errors_on_inline_std_fs_call() {
    let content = "fn main() { let _ = std::fs::read_to_string(\"foo\"); }";
    let raw_results = check_source("src/foo.rs", content, false);
    let results = findings(&raw_results);

    assert_normalized_len(&results, 1);
    assert_eq!(results[0].id, "RS-CODE-15");
    assert_eq!(results[0].severity, Severity::Error);
    assert_eq!(results[0].file.as_deref(), Some("src/foo.rs"));
    assert_eq!(results[0].line, Some(1));
    assert_eq!(results[0].title, "direct std::fs call");
    assert_eq!(
        results[0].message,
        "Direct `std::fs::*` call found: `fn main() { let _ = std::fs::read_to_string(\"foo\"); }`."
    );
}

#[test]
fn still_errors_inside_allow_scoped_std_fs_usage() {
    let content = "#[allow(clippy::disallowed_methods)]\nfn main() { let _ = std::fs::read_to_string(\"foo\"); }";
    let raw_results = check_source("src/foo.rs", content, false);
    let results = findings(&raw_results);

    assert_normalized_len(&results, 1);
    assert_eq!(results[0].id, "RS-CODE-15");
    assert_eq!(results[0].severity, Severity::Error);
    assert_eq!(results[0].title, "direct std::fs call");
    assert_eq!(results[0].line, Some(2));
}

#[test]
fn errors_on_std_alias_fs_call() {
    let content = "use std as s;\nfn main() { let _ = s::fs::read_to_string(\"foo\"); }";
    let raw_results = check_source("src/foo.rs", content, false);
    let results = findings(&raw_results);

    assert_normalized_len(&results, 1);
    assert_eq!(results[0].id, "RS-CODE-15");
    assert_eq!(results[0].severity, Severity::Error);
    assert_eq!(results[0].title, "direct std::fs call");
    assert_eq!(results[0].line, Some(2));
}

#[test]
fn errors_on_extern_crate_std_alias_fs_call() {
    let content =
        "extern crate std as s;\nfn main() { let _ = s::fs::read_to_string(\"foo\"); }";
    let raw_results = check_source("src/foo.rs", content, false);
    let results = findings(&raw_results);

    assert_normalized_len(&results, 1);
    assert_eq!(results[0].id, "RS-CODE-15");
    assert_eq!(results[0].severity, Severity::Error);
    assert_eq!(results[0].title, "direct std::fs call");
    assert_eq!(results[0].line, Some(2));
}

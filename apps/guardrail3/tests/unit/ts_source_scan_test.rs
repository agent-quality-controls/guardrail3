//! Tests extracted from `app::ts::validate::source_scan`
#![allow(
    clippy::expect_used,
    clippy::disallowed_methods,
    clippy::indexing_slicing,
    clippy::panic,
    clippy::manual_assert
)] // reason: test crate

use std::path::Path;

use guardrail3::app::ts::validate::source_scan::{
    check_any_types, check_comment_pattern, check_file_length, check_process_env,
};
use guardrail3::domain::report::Severity;

// T30: process.env direct access
#[test]
fn test_process_env_direct_access_t30() {
    let path = Path::new("src/app.ts");
    let content = "const x = process.env.NODE_ENV;\n";
    let mut results = Vec::new();
    check_process_env(path, content, &mut results);
    assert_eq!(results.len(), 1, "expected 1 result, got {results:?}");
    assert_eq!(results[0].id, "T30");
    assert!(matches!(results[0].severity, Severity::Error));
}

// T31: any type usage
#[test]
fn test_any_type_usage_t31() {
    let path = Path::new("src/app.ts");
    let content = "const x: any = 5;\nconst y = foo as any;\n";
    let mut results = Vec::new();
    check_any_types(path, content, &mut results);
    assert!(
        !results.is_empty(),
        "expected at least 1 result for any type usage"
    );
    for r in &results {
        assert_eq!(r.id, "T31");
        assert!(matches!(r.severity, Severity::Info));
    }
}

// T32: file length over 400 effective lines
#[test]
fn test_file_length_over_400_t32() {
    let path = Path::new("src/big.ts");
    let content: String = (0..410)
        .map(|i| format!("const x{i} = {i};"))
        .collect::<Vec<_>>()
        .join("\n");
    let mut results = Vec::new();
    check_file_length(path, &content, &mut results);
    assert_eq!(results.len(), 1, "expected 1 result, got {results:?}");
    assert_eq!(results[0].id, "T32");
    assert!(matches!(results[0].severity, Severity::Error));
}

// T33 was killed — files between 250-300 lines no longer produce a warning.
// Only T32 (>300 effective lines) remains.
#[test]
fn test_file_length_250_to_300_no_warning() {
    let path = Path::new("src/medium.ts");
    let content: String = (0..260)
        .map(|i| format!("const x{i} = {i};"))
        .collect::<Vec<_>>()
        .join("\n");
    let mut results = Vec::new();
    check_file_length(path, &content, &mut results);
    assert_eq!(
        results.len(),
        0,
        "expected no results for 260 lines, got {results:?}"
    );
}

// T34: noinspection comment
#[test]
fn test_noinspection_comment_t34() {
    let path = Path::new("src/app.ts");
    let content = "// noinspection TypeScriptValidateTypes\nconst x = 1;\n";
    let mut results = Vec::new();
    check_comment_pattern(
        path,
        content,
        &["// noinspection", "/* noinspection"],
        "T34",
        "noinspection comment",
        &mut results,
    );
    assert_eq!(results.len(), 1, "expected 1 result, got {results:?}");
    assert_eq!(results[0].id, "T34");
    assert!(matches!(results[0].severity, Severity::Info));
}

// T35: istanbul ignore
#[test]
fn test_istanbul_ignore_t35() {
    let path = Path::new("src/app.ts");
    let content = "/* istanbul ignore next */\nfunction foo() {}\n";
    let mut results = Vec::new();
    check_comment_pattern(
        path,
        content,
        &["istanbul ignore", "c8 ignore"],
        "T35",
        "Coverage ignore comment",
        &mut results,
    );
    assert_eq!(results.len(), 1, "expected 1 result, got {results:?}");
    assert_eq!(results[0].id, "T35");
    assert!(matches!(results[0].severity, Severity::Info));
}

#![allow(
    clippy::expect_used,
    clippy::missing_const_for_fn,
    clippy::missing_panics_doc,
    reason = "assertion helpers are reusable panic-based proof sites for test harnesses"
)]

use guardrail3_check_types::{G3CheckResult, G3Severity};

pub fn assert_code_ast_results(lib_results: &[G3CheckResult], test_results: &[G3CheckResult]) {
    assert!(
        lib_results
            .iter()
            .any(|result| result.id() == "g3rs-code/todo-macros"),
        "lib input should preserve todo! detection: {lib_results:#?}"
    );
    assert!(
        test_results.is_empty(),
        "test-owned source should preserve current no-findings behavior for the migrated rules: {test_results:#?}"
    );
}

pub fn assert_results_empty(results: &[G3CheckResult]) {
    assert!(results.is_empty(), "{results:#?}");
}

pub fn assert_result_count(results: &[G3CheckResult], expected: usize) {
    assert_eq!(results.len(), expected, "{results:#?}");
}

pub fn assert_file_result_count(results: &[G3CheckResult], file: &str, expected: usize) {
    let actual = results
        .iter()
        .filter(|result| result.file() == Some(file))
        .count();
    assert_eq!(actual, expected, "{results:#?}");
}

pub fn assert_has_result_id(results: &[G3CheckResult], file: &str, id: &str) {
    assert!(
        results
            .iter()
            .any(|result| result.file() == Some(file) && result.id() == id),
        "{results:#?}"
    );
}

pub fn assert_has_result_id_with_severity(
    results: &[G3CheckResult],
    file: &str,
    id: &str,
    severity: G3Severity,
) {
    assert!(
        results.iter().any(|result| {
            result.file() == Some(file) && result.id() == id && result.severity() == severity
        }),
        "{results:#?}"
    );
}

pub fn assert_no_results_for_file(results: &[G3CheckResult], file: &str) {
    assert!(
        !results.iter().any(|result| result.file() == Some(file)),
        "{results:#?}"
    );
}

pub fn assert_single_parse_failed_error(err: &g3rs_code_ingestion_runtime::IngestionError) {
    assert!(
        matches!(
            err,
            g3rs_code_ingestion_runtime::IngestionError::ParseFailed { .. }
        ),
        "unexpected error: {err:?}"
    );
}

pub fn assert_single_unreadable_error(err: &g3rs_code_ingestion_runtime::IngestionError) {
    assert!(
        matches!(
            err,
            g3rs_code_ingestion_runtime::IngestionError::Unreadable { .. }
        ),
        "unexpected error: {err:?}"
    );
}

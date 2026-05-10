use guardrail3_check_types::{G3CheckResult, G3Severity};

/// Assert that the result is the inventory `info` for `unsafe_code = forbid`.
///
/// # Panics
///
/// Panics when any expected field does not match.
pub fn assert_forbid_inventory_info(result: &G3CheckResult, rel_path: &str) {
    assert_eq!(
        result.id(),
        "g3rs-code/unsafe-code-lint",
        "unexpected id: {result:#?}"
    );
    assert_eq!(
        result.severity(),
        G3Severity::Info,
        "unexpected severity: {result:#?}"
    );
    assert_eq!(
        result.title(),
        "unsafe_code = forbid",
        "unexpected title: {result:#?}"
    );
    assert_eq!(
        result.message(),
        "unsafe_code is set to forbid in workspace lints.",
        "unexpected message: {result:#?}"
    );
    assert_eq!(
        result.file(),
        Some(rel_path),
        "unexpected file: {result:#?}"
    );
    assert_eq!(result.line(), None, "unexpected line: {result:#?}");
    assert!(
        result.inventory(),
        "forbid inventory should stay hidden by default"
    );
}

/// Assert that the result is an `error` for `unsafe_code = deny`.
///
/// # Panics
///
/// Panics when any expected field does not match.
pub fn assert_deny_error(result: &G3CheckResult, rel_path: &str) {
    assert_eq!(
        result.id(),
        "g3rs-code/unsafe-code-lint",
        "unexpected id: {result:#?}"
    );
    assert_eq!(
        result.severity(),
        G3Severity::Error,
        "unexpected severity: {result:#?}"
    );
    assert_eq!(
        result.title(),
        "unsafe_code should be forbid",
        "unexpected title: {result:#?}"
    );
    assert_eq!(
        result.message(),
        "unsafe_code = deny can be overridden; use forbid in workspace lints.",
        "unexpected message: {result:#?}"
    );
    assert_eq!(
        result.file(),
        Some(rel_path),
        "unexpected file: {result:#?}"
    );
    assert_eq!(result.line(), None, "unexpected line: {result:#?}");
    assert!(!result.inventory(), "deny should stay in normal output");
}

/// Assert that exactly one result is the forbid inventory info.
///
/// # Panics
///
/// Panics when results length is not 1 or shape mismatches.
pub fn assert_single_forbid_inventory_info(results: &[G3CheckResult], rel_path: &str) {
    assert_eq!(results.len(), 1, "unexpected results: {results:#?}");
    let [result] = results else { return };
    assert_forbid_inventory_info(result, rel_path);
}

/// Assert that exactly one result is the deny error.
///
/// # Panics
///
/// Panics when results length is not 1 or shape mismatches.
pub fn assert_single_deny_error(results: &[G3CheckResult], rel_path: &str) {
    assert_eq!(results.len(), 1, "unexpected results: {results:#?}");
    let [result] = results else { return };
    assert_deny_error(result, rel_path);
}

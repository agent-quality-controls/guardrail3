use guardrail3_check_types::{G3CheckResult, G3Severity};

/// Assert a single exception comment inventory warn matches the expected shape.
///
/// # Panics
///
/// Panics when any expected field does not match.
pub fn assert_inventory_warn(result: &G3CheckResult, rel_path: &str, line: usize, line_text: &str) {
    assert_eq!(
        result.id(),
        "g3rs-code/exception-comment-inventory",
        "unexpected id: {result:#?}"
    );
    assert_eq!(
        result.severity(),
        G3Severity::Warn,
        "unexpected severity: {result:#?}"
    );
    assert_eq!(
        result.title(),
        "EXCEPTION comment inventory",
        "unexpected title: {result:#?}"
    );
    assert_eq!(
        result.message(),
        format!("Config exception comment: {line_text}"),
        "unexpected message: {result:#?}"
    );
    assert_eq!(
        result.file(),
        Some(rel_path),
        "unexpected file: {result:#?}"
    );
    assert_eq!(result.line(), Some(line), "unexpected line: {result:#?}");
    assert!(!result.inventory(), "rule 7 stays visible in normal output");
}

/// Assert exactly two exception comment inventory warns match the expected shapes.
///
/// # Panics
///
/// Panics when there are not exactly two results, or when either fails to match.
pub fn assert_two_inventory_warns(
    results: &[G3CheckResult],
    first_rel_path: &str,
    first_line: usize,
    first_line_text: &str,
    second_rel_path: &str,
    second_line: usize,
    second_line_text: &str,
) {
    assert_eq!(results.len(), 2, "{results:#?}");
    let [first, second] = results else { return };
    assert_inventory_warn(first, first_rel_path, first_line, first_line_text);
    assert_inventory_warn(second, second_rel_path, second_line, second_line_text);
}

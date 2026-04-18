use guardrail3_check_types::{G3CheckResult, G3Severity};

pub fn assert_inventory_warn(result: &G3CheckResult, rel_path: &str, line: usize, line_text: &str) {
    assert_eq!(result.id(), "RS-CODE-CONFIG-07");
    assert_eq!(result.severity(), G3Severity::Warn);
    assert_eq!(result.title(), "EXCEPTION comment inventory");
    assert_eq!(
        result.message(),
        format!("Config exception comment: {line_text}")
    );
    assert_eq!(result.file(), Some(rel_path));
    assert_eq!(result.line(), Some(line));
    assert!(!result.inventory(), "rule 7 stays visible in normal output");
}

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
    assert_inventory_warn(&results[0], first_rel_path, first_line, first_line_text);
    assert_inventory_warn(&results[1], second_rel_path, second_line, second_line_text);
}

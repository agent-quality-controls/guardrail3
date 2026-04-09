use guardrail3_check_types::{G3CheckResult, G3Severity};

pub fn assert_inventory_warn(
    result: &G3CheckResult,
    rel_path: &str,
    line: usize,
    line_text: &str,
) {
    assert_eq!(result.id(), "RS-CODE-07");
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

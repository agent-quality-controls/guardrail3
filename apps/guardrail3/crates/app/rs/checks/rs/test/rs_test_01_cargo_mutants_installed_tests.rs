use crate::domain::report::Severity;

use super::super::test_support::tool_input;
use super::check;

#[test]
fn inventories_installed_tool() {
    let input = tool_input(true);
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-TEST-01");
    assert_eq!(results[0].severity, Severity::Info);
    assert!(results[0].inventory);
}

#[test]
fn warns_when_tool_missing() {
    let input = tool_input(false);
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].severity, Severity::Warn);
    assert!(!results[0].inventory);
}

use crate::domain::report::Severity;

use super::super::test_support::{tool_facts, tool_input};
use super::check;

#[test]
fn inventories_installed_cargo_deny() {
    let facts = tool_facts("cargo-deny", true);
    let input = tool_input(&facts, "cargo-deny");
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-DEPS-01");
    assert_eq!(result.severity, Severity::Info);
    assert!(result.inventory);
    assert_eq!(result.title, "cargo-deny installed");
}

#[test]
fn errors_when_cargo_deny_missing() {
    let facts = tool_facts("cargo-deny", false);
    let input = tool_input(&facts, "cargo-deny");
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-DEPS-01");
    assert_eq!(result.severity, Severity::Error);
    assert!(!result.inventory);
    assert_eq!(result.title, "cargo-deny missing");
}

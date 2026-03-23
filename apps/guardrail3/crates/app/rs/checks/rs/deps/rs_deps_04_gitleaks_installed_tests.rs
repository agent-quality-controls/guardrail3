use crate::domain::report::Severity;

use super::super::test_support::{tool_facts, tool_input};
use super::check;

#[test]
fn inventories_installed_gitleaks() {
    let facts = tool_facts("gitleaks", true);
    let input = tool_input(&facts, "gitleaks");
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].severity, Severity::Info);
    assert!(results[0].inventory);
}

#[test]
fn errors_when_gitleaks_missing() {
    let facts = tool_facts("gitleaks", false);
    let input = tool_input(&facts, "gitleaks");
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-DEPS-04");
    assert_eq!(results[0].severity, Severity::Error);
    assert_eq!(results[0].title, "gitleaks missing");
}

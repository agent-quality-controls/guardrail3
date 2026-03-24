use crate::app::rs::checks::rs::deps::test_support::{tool_facts, tool_input};
use crate::domain::report::Severity;

#[test]
fn inventories_installed_gitleaks() {
    let facts = tool_facts("gitleaks", true);
    let input = tool_input(&facts, "gitleaks");
    let mut results = Vec::new();

    super::super::check(&input, &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-DEPS-04");
    assert_eq!(result.severity, Severity::Info);
    assert!(result.inventory);
    assert_eq!(result.title, "gitleaks installed");
}

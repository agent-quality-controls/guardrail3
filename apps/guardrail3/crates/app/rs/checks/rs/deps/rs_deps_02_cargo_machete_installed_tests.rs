use crate::domain::report::Severity;

use super::super::test_support::{tool_facts, tool_input};
use super::check;

#[test]
fn inventories_installed_cargo_machete() {
    let facts = tool_facts("cargo-machete", true);
    let input = tool_input(&facts, "cargo-machete");
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-DEPS-02");
    assert_eq!(results[0].severity, Severity::Info);
    assert!(results[0].inventory);
}

#[test]
fn errors_when_cargo_machete_missing() {
    let facts = tool_facts("cargo-machete", false);
    let input = tool_input(&facts, "cargo-machete");
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-DEPS-02");
    assert_eq!(results[0].severity, Severity::Error);
    assert_eq!(results[0].title, "cargo-machete missing");
}

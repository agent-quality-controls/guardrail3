use crate::domain::report::Severity;

use super::super::test_support::{tool_facts, tool_input};
use super::check;

#[test]
fn inventories_installed_cargo_dupes() {
    let facts = tool_facts("cargo-dupes", true);
    let input = tool_input(&facts, "cargo-dupes");
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].severity, Severity::Info);
    assert!(results[0].inventory);
}

#[test]
fn warns_when_cargo_dupes_missing() {
    let facts = tool_facts("cargo-dupes", false);
    let input = tool_input(&facts, "cargo-dupes");
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-DEPS-03");
    assert_eq!(results[0].severity, Severity::Warn);
    assert_eq!(results[0].title, "cargo-dupes missing");
}

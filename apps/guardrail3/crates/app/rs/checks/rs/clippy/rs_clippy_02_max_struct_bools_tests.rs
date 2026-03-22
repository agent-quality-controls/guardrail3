use crate::domain::report::Severity;

use super::super::inputs::ConfigClippyInput;
use super::super::test_support::{collected_facts, root_workspace_tree};
use super::check;

#[test]
fn inventories_max_struct_bools_when_correct() {
    let tree = root_workspace_tree("max-struct-bools = 3");
    let facts = collected_facts(&tree);
    let input = ConfigClippyInput::new(facts.allowed_configs.first().expect("config"));
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-CLIPPY-02");
    assert!(result.inventory);
    assert_eq!(result.severity, Severity::Info);
    assert_eq!(result.title, "max-struct-bools correct");
    assert_eq!(result.message, "max-struct-bools = 3");
}

#[test]
fn errors_when_max_struct_bools_is_wrong() {
    let tree = root_workspace_tree("max-struct-bools = 4");
    let facts = collected_facts(&tree);
    let input = ConfigClippyInput::new(facts.allowed_configs.first().expect("config"));
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-CLIPPY-02");
    assert_eq!(result.severity, Severity::Error);
    assert_eq!(result.title, "max-struct-bools wrong value");
    assert_eq!(result.message, "Expected 3, got 4.");
}

#[test]
fn errors_when_max_struct_bools_is_missing() {
    let tree = root_workspace_tree("");
    let facts = collected_facts(&tree);
    let input = ConfigClippyInput::new(facts.allowed_configs.first().expect("config"));
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-CLIPPY-02");
    assert_eq!(result.severity, Severity::Error);
    assert_eq!(result.title, "max-struct-bools missing");
    assert_eq!(result.message, "Expected max-struct-bools = 3.");
}

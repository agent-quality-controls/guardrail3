use crate::domain::report::Severity;

use super::super::inputs::ConfigClippyInput;
use super::super::test_support::{collected_facts, root_workspace_tree};
use super::check;

#[test]
fn inventories_type_complexity_threshold_when_correct() {
    let tree = root_workspace_tree("type-complexity-threshold = 75");
    let facts = collected_facts(&tree);
    let input = ConfigClippyInput::new(facts.allowed_configs.first().expect("config"));
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results.iter().any(|result| {
        result.id == "RS-CLIPPY-22"
            && result.inventory
            && result.severity == Severity::Info
            && result.title == "type-complexity-threshold correct"
            && result.message == "type-complexity-threshold = 75"
    }));
}

#[test]
fn errors_when_type_complexity_threshold_is_wrong() {
    let tree = root_workspace_tree("type-complexity-threshold = 76");
    let facts = collected_facts(&tree);
    let input = ConfigClippyInput::new(facts.allowed_configs.first().expect("config"));
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results.iter().any(|result| {
        result.id == "RS-CLIPPY-22"
            && result.severity == Severity::Error
            && result.title == "type-complexity-threshold wrong value"
            && result.message == "Expected 75, got 76."
    }));
}

#[test]
fn errors_when_type_complexity_threshold_is_missing() {
    let tree = root_workspace_tree("");
    let facts = collected_facts(&tree);
    let input = ConfigClippyInput::new(facts.allowed_configs.first().expect("config"));
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results.iter().any(|result| {
        result.id == "RS-CLIPPY-22"
            && result.severity == Severity::Error
            && result.title == "type-complexity-threshold missing"
            && result.message == "Expected type-complexity-threshold = 75."
    }));
}

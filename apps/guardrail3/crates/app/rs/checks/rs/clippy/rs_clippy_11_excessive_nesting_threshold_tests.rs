use crate::domain::report::Severity;

use super::super::inputs::ConfigClippyInput;
use super::super::test_support::{collected_facts, root_workspace_tree};
use super::check;

#[test]
fn inventories_excessive_nesting_threshold_when_correct() {
    let tree = root_workspace_tree("excessive-nesting-threshold = 4");
    let facts = collected_facts(&tree);
    let input = ConfigClippyInput::new(facts.allowed_configs.first().expect("config"));
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results.iter().any(|result| {
        result.id == "RS-CLIPPY-11"
            && result.inventory
            && result.severity == Severity::Info
            && result.title == "excessive-nesting-threshold correct"
            && result.message == "excessive-nesting-threshold = 4"
    }));
}

#[test]
fn errors_when_excessive_nesting_threshold_is_wrong() {
    let tree = root_workspace_tree("excessive-nesting-threshold = 5");
    let facts = collected_facts(&tree);
    let input = ConfigClippyInput::new(facts.allowed_configs.first().expect("config"));
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results.iter().any(|result| {
        result.id == "RS-CLIPPY-11"
            && result.severity == Severity::Error
            && result.title == "excessive-nesting-threshold wrong value"
            && result.message == "Expected 4, got 5."
    }));
}

#[test]
fn errors_when_excessive_nesting_threshold_is_missing() {
    let tree = root_workspace_tree("");
    let facts = collected_facts(&tree);
    let input = ConfigClippyInput::new(facts.allowed_configs.first().expect("config"));
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results.iter().any(|result| {
        result.id == "RS-CLIPPY-11"
            && result.severity == Severity::Error
            && result.title == "excessive-nesting-threshold missing"
            && result.message == "Expected excessive-nesting-threshold = 4."
    }));
}

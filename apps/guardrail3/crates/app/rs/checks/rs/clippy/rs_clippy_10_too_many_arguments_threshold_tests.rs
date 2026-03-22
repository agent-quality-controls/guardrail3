use crate::domain::report::Severity;

use super::super::inputs::ConfigClippyInput;
use super::super::test_support::{collected_facts, root_workspace_tree};
use super::check;

#[test]
fn inventories_too_many_arguments_threshold_when_correct() {
    let tree = root_workspace_tree("too-many-arguments-threshold = 7");
    let facts = collected_facts(&tree);
    let input = ConfigClippyInput::new(facts.allowed_configs.first().expect("config"));
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results.iter().any(|result| {
        result.id == "RS-CLIPPY-10"
            && result.inventory
            && result.severity == Severity::Info
            && result.title == "too-many-arguments-threshold correct"
            && result.message == "too-many-arguments-threshold = 7"
    }));
}

#[test]
fn errors_when_too_many_arguments_threshold_is_wrong() {
    let tree = root_workspace_tree("too-many-arguments-threshold = 8");
    let facts = collected_facts(&tree);
    let input = ConfigClippyInput::new(facts.allowed_configs.first().expect("config"));
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results.iter().any(|result| {
        result.id == "RS-CLIPPY-10"
            && result.severity == Severity::Error
            && result.title == "too-many-arguments-threshold wrong value"
            && result.message == "Expected 7, got 8."
    }));
}

#[test]
fn errors_when_too_many_arguments_threshold_is_missing() {
    let tree = root_workspace_tree("");
    let facts = collected_facts(&tree);
    let input = ConfigClippyInput::new(facts.allowed_configs.first().expect("config"));
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results.iter().any(|result| {
        result.id == "RS-CLIPPY-10"
            && result.severity == Severity::Error
            && result.title == "too-many-arguments-threshold missing"
            && result.message == "Expected too-many-arguments-threshold = 7."
    }));
}

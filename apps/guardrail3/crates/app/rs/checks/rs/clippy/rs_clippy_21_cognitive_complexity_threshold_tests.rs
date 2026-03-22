use crate::domain::report::Severity;

use super::super::inputs::ConfigClippyInput;
use super::super::test_support::{collected_facts, root_workspace_tree};
use super::check;

#[test]
fn inventories_cognitive_complexity_threshold_when_correct() {
    let tree = root_workspace_tree("cognitive-complexity-threshold = 15");
    let facts = collected_facts(&tree);
    let input = ConfigClippyInput::new(facts.allowed_configs.first().expect("config"));
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results.iter().any(|result| {
        result.id == "RS-CLIPPY-21"
            && result.inventory
            && result.severity == Severity::Info
            && result.title == "cognitive-complexity-threshold correct"
            && result.message == "cognitive-complexity-threshold = 15"
    }));
}

#[test]
fn errors_when_cognitive_complexity_threshold_is_wrong() {
    let tree = root_workspace_tree("cognitive-complexity-threshold = 16");
    let facts = collected_facts(&tree);
    let input = ConfigClippyInput::new(facts.allowed_configs.first().expect("config"));
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results.iter().any(|result| {
        result.id == "RS-CLIPPY-21"
            && result.severity == Severity::Error
            && result.title == "cognitive-complexity-threshold wrong value"
            && result.message == "Expected 15, got 16."
    }));
}

#[test]
fn errors_when_cognitive_complexity_threshold_is_missing() {
    let tree = root_workspace_tree("");
    let facts = collected_facts(&tree);
    let input = ConfigClippyInput::new(facts.allowed_configs.first().expect("config"));
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results.iter().any(|result| {
        result.id == "RS-CLIPPY-21"
            && result.severity == Severity::Error
            && result.title == "cognitive-complexity-threshold missing"
            && result.message == "Expected cognitive-complexity-threshold = 15."
    }));
}

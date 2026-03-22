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
    assert!(results.iter().any(|result| {
        result.id == "RS-CLIPPY-02"
            && result.inventory
            && result.severity == Severity::Info
            && result.title == "max-struct-bools correct"
            && result.message == "max-struct-bools = 3"
    }));
}

#[test]
fn errors_when_max_struct_bools_is_wrong() {
    let tree = root_workspace_tree("max-struct-bools = 4");
    let facts = collected_facts(&tree);
    let input = ConfigClippyInput::new(facts.allowed_configs.first().expect("config"));
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results.iter().any(|result| {
        result.id == "RS-CLIPPY-02"
            && result.severity == Severity::Error
            && result.title == "max-struct-bools wrong value"
            && result.message == "Expected 3, got 4."
    }));
}

#[test]
fn errors_when_max_struct_bools_is_missing() {
    let tree = root_workspace_tree("");
    let facts = collected_facts(&tree);
    let input = ConfigClippyInput::new(facts.allowed_configs.first().expect("config"));
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results.iter().any(|result| {
        result.id == "RS-CLIPPY-02"
            && result.severity == Severity::Error
            && result.title == "max-struct-bools missing"
            && result.message == "Expected max-struct-bools = 3."
    }));
}

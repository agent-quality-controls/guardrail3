use crate::domain::report::Severity;

use super::super::inputs::ConfigClippyInput;
use super::super::test_support::{collected_facts, root_workspace_tree};
use super::check;

#[test]
fn inventories_max_fn_params_bools_when_correct() {
    let tree = root_workspace_tree("max-fn-params-bools = 3");
    let facts = collected_facts(&tree);
    let input = ConfigClippyInput::new(facts.allowed_configs.first().expect("config"));
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results.iter().any(|result| {
        result.id == "RS-CLIPPY-03"
            && result.inventory
            && result.severity == Severity::Info
            && result.title == "max-fn-params-bools correct"
            && result.message == "max-fn-params-bools = 3"
    }));
}

#[test]
fn errors_when_max_fn_params_bools_is_wrong() {
    let tree = root_workspace_tree("max-fn-params-bools = 4");
    let facts = collected_facts(&tree);
    let input = ConfigClippyInput::new(facts.allowed_configs.first().expect("config"));
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results.iter().any(|result| {
        result.id == "RS-CLIPPY-03"
            && result.severity == Severity::Error
            && result.title == "max-fn-params-bools wrong value"
            && result.message == "Expected 3, got 4."
    }));
}

#[test]
fn errors_when_max_fn_params_bools_is_missing() {
    let tree = root_workspace_tree("");
    let facts = collected_facts(&tree);
    let input = ConfigClippyInput::new(facts.allowed_configs.first().expect("config"));
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results.iter().any(|result| {
        result.id == "RS-CLIPPY-03"
            && result.severity == Severity::Error
            && result.title == "max-fn-params-bools missing"
            && result.message == "Expected max-fn-params-bools = 3."
    }));
}

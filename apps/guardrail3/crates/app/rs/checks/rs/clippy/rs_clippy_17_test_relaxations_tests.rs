use crate::domain::report::Severity;

use super::super::inputs::ConfigClippyInput;
use super::super::test_support::{collected_facts, root_workspace_tree};
use super::check;

#[test]
fn warns_when_test_relaxations_are_enabled() {
    let tree = root_workspace_tree("allow-dbg-in-tests = true");
    let facts = collected_facts(&tree);
    let input = ConfigClippyInput::new(facts.allowed_configs.first().expect("config"));
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results.iter().any(|result| {
        result.id == "RS-CLIPPY-17"
            && result.severity == Severity::Warn
            && result.title == "clippy test relaxation enabled"
            && result.message == "`allow-dbg-in-tests = true` relaxes test output discipline."
    }));
}

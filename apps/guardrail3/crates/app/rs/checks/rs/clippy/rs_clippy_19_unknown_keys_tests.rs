use crate::domain::report::Severity;

use super::super::inputs::ConfigClippyInput;
use super::super::test_support::{collected_facts, root_workspace_tree};
use super::check;

#[test]
fn warns_on_managed_key_typos() {
    let tree = root_workspace_tree("disalowed-methods = []");
    let facts = collected_facts(&tree);
    let input = ConfigClippyInput::new(facts.allowed_configs.first().expect("config"));
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-CLIPPY-19");
    assert_eq!(result.severity, Severity::Warn);
    assert_eq!(result.title, "unrecognized clippy.toml key");
    assert_eq!(
        result.message,
        "Top-level key `disalowed-methods` looks like a typo of a guardrail-managed clippy key."
    );
}

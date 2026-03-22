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
    assert!(results.iter().any(|result| {
        result.id == "RS-CLIPPY-19"
            && result.severity == Severity::Warn
            && result.title == "unrecognized clippy.toml key"
            && result.message.contains("disalowed-methods")
    }));
}

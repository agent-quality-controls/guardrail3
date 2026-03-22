use crate::domain::report::Severity;

use super::super::inputs::ConfigClippyInput;
use super::super::test_support::{collected_facts, root_workspace_tree};
use super::check;

#[test]
fn warns_on_duplicate_bans() {
    let tree = root_workspace_tree(
        r#"
disallowed-macros = [
    { path = "println", reason = "good enough reason text" },
    { path = "println", reason = "another good enough reason text" },
]
"#,
    );
    let facts = collected_facts(&tree);
    let input = ConfigClippyInput::new(facts.allowed_configs.first().expect("config"));
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-CLIPPY-18");
    assert_eq!(result.severity, Severity::Warn);
    assert_eq!(result.title, "duplicate ban entry");
    assert_eq!(result.message, "`println` appears 2 times in `disallowed-macros`.");
}

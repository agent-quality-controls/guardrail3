use crate::domain::report::Severity;

use super::super::inputs::ConfigClippyInput;
use super::super::test_support::{collected_facts, root_workspace_tree};
use super::check;

#[test]
fn warns_on_placeholder_reasons() {
    let tree = root_workspace_tree(
        r#"
disallowed-methods = [
    { path = "std::env::var", reason = "todo" },
]
"#,
    );
    let facts = collected_facts(&tree);
    let input = ConfigClippyInput::new(facts.allowed_configs.first().expect("config"));
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-CLIPPY-15");
    assert_eq!(result.severity, Severity::Warn);
    assert_eq!(result.title, "ban entry has placeholder reason");
    assert_eq!(
        result.message,
        "`std::env::var` in `disallowed-methods` has a trivial or placeholder `reason`."
    );
}

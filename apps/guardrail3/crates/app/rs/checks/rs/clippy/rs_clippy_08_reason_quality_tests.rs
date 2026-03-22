use crate::domain::report::Severity;

use super::super::test_support::{collected_facts, config_input, root_workspace_tree};
use super::check;

#[test]
fn warns_on_missing_reason_entries() {
    let tree = root_workspace_tree(r#"disallowed-methods = ["std::env::var"]"#);
    let facts = collected_facts(&tree);
    let mut results = Vec::new();
    check(&config_input(&facts, "clippy.toml"), &mut results);
    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-CLIPPY-08");
    assert!(!result.inventory);
    assert_eq!(result.severity, Severity::Warn);
    assert_eq!(result.title, "ban entry missing reason");
    assert_eq!(
        result.message,
        "`std::env::var` in `disallowed-methods` must use table format with a `reason` field."
    );
}

use crate::domain::report::Severity;

use super::super::test_support::{collected_facts, config_input, incomplete_workspace_policy_root_tree};
use super::check;

#[test]
fn errors_when_local_policy_root_drops_baseline() {
    let tree = incomplete_workspace_policy_root_tree();
    let facts = collected_facts(&tree);
    let mut results = Vec::new();
    check(&config_input(&facts, "workspace/clippy.toml"), &mut results);
    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-CLIPPY-13");
    assert!(!result.inventory);
    assert_eq!(result.severity, Severity::Error);
    assert_eq!(result.title, "local clippy policy root drops managed baseline");
    assert_eq!(result.file.as_deref(), Some("workspace/clippy.toml"));
    assert!(result.message.contains("thresholds"));
}

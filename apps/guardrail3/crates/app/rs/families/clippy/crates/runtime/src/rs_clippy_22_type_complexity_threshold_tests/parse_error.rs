use guardrail3_domain_report::Severity;

use super::super::super::test_support::{collected_facts, config_input, root_workspace_tree};
use super::super::check;

#[test]
fn errors_when_clippy_config_cannot_be_parsed() {
    let tree = root_workspace_tree("[");
    let facts = collected_facts(&tree);
    let mut results = Vec::new();

    check(&config_input(&facts, "clippy.toml"), &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-CLIPPY-22");
    assert_eq!(result.severity, Severity::Error);
    assert_eq!(result.title, "clippy.toml parse error");
    assert!(result.message.starts_with("Failed to parse clippy.toml: "));
    assert_eq!(result.file.as_deref(), Some("clippy.toml"));
}

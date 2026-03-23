use crate::domain::report::Severity;

use super::super::super::test_support::{collected_facts, config_input, root_workspace_tree};
use super::super::check;

#[test]
fn errors_when_excessive_nesting_threshold_is_wrong() {
    let tree = root_workspace_tree("excessive-nesting-threshold = 5");
    let facts = collected_facts(&tree);
    let mut results = Vec::new();

    check(&config_input(&facts, "clippy.toml"), &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-CLIPPY-11");
    assert_eq!(result.severity, Severity::Error);
    assert_eq!(result.title, "excessive-nesting-threshold wrong value");
    assert_eq!(result.message, "Expected 4, got 5.");
}

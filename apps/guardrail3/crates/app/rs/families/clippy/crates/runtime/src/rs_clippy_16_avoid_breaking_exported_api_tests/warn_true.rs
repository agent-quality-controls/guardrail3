use guardrail3_domain_report::Severity;

use super::super::super::test_support::{collected_facts, config_input, root_workspace_tree};
use super::super::check;

#[test]
fn warns_when_avoid_breaking_exported_api_is_true_for_non_publishable_roots() {
    let tree = root_workspace_tree("avoid-breaking-exported-api = true");
    let facts = collected_facts(&tree);
    let mut results = Vec::new();

    check(&config_input(&facts, "clippy.toml"), &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-CLIPPY-16");
    assert_eq!(result.severity, Severity::Warn);
    assert_eq!(result.title, "avoid-breaking-exported-api enabled");
    assert_eq!(
        result.message,
        "`avoid-breaking-exported-api = true` suppresses useful lints. Prefer `false`."
    );
    assert!(!result.inventory);
}

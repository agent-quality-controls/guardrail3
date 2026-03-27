use guardrail3_domain_report::Severity;

use super::super::super::test_support::{collected_facts, config_input, root_workspace_tree};
use super::super::check;

#[test]
fn warns_when_setting_is_missing() {
    let tree = root_workspace_tree("");
    let facts = collected_facts(&tree);
    let mut results = Vec::new();

    check(&config_input(&facts, "clippy.toml"), &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-CLIPPY-16");
    assert_eq!(result.severity, Severity::Warn);
    assert_eq!(result.title, "avoid-breaking-exported-api not set");
    assert_eq!(
        result.message,
        "Set `avoid-breaking-exported-api = false` explicitly unless this is a published library."
    );
    assert!(!result.inventory);
}

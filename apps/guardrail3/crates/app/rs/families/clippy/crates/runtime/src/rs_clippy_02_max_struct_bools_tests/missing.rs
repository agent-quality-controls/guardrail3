use guardrail3_domain_report::Severity;

use super::super::super::test_support::{collected_facts, config_input, root_workspace_tree};
use super::super::check;

#[test]
fn errors_when_max_struct_bools_is_missing() {
    let tree = root_workspace_tree("");
    let facts = collected_facts(&tree);
    let mut results = Vec::new();

    check(&config_input(&facts, "clippy.toml"), &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-CLIPPY-02");
    assert_eq!(result.severity, Severity::Error);
    assert_eq!(result.title, "max-struct-bools missing");
    assert_eq!(result.message, "Expected max-struct-bools = 3.");
}

use guardrail3_domain_report::Severity;

use super::super::super::test_support::{
    canonical_clippy_toml, collected_facts, config_input, root_workspace_tree,
};
use super::super::check;

#[test]
fn inventories_generated_max_struct_bools_baseline() {
    let tree = root_workspace_tree(canonical_clippy_toml());
    let facts = collected_facts(&tree);
    let mut results = Vec::new();

    check(&config_input(&facts, "clippy.toml"), &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-CLIPPY-02");
    assert!(result.inventory);
    assert_eq!(result.severity, Severity::Info);
    assert_eq!(result.title, "max-struct-bools correct");
    assert_eq!(result.message, "max-struct-bools = 3");
    assert_eq!(result.file.as_deref(), Some("clippy.toml"));
}

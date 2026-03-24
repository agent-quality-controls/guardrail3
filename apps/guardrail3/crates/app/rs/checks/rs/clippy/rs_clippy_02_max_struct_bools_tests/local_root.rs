use crate::domain::report::Severity;

use super::super::super::test_support::{
    canonical_clippy_toml, collected_facts, config_input, library_workspace_root_tree,
};
use super::super::check;

#[test]
fn inventories_generated_threshold_at_a_local_policy_root_too() {
    let tree = library_workspace_root_tree(canonical_clippy_toml());
    let facts = collected_facts(&tree);
    let mut results = Vec::new();

    check(
        &config_input(&facts, "apps/libsite/clippy.toml"),
        &mut results,
    );

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-CLIPPY-02");
    assert_eq!(result.severity, Severity::Info);
    assert!(result.inventory);
    assert_eq!(result.file.as_deref(), Some("apps/libsite/clippy.toml"));
    assert_eq!(result.title, "max-struct-bools correct");
}

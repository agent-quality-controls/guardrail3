use guardrail3_domain_modules::clippy::build_clippy_toml;
use guardrail3_domain_report::Severity;

use super::super::super::test_support::{
    collected_facts, config_input, library_workspace_root_tree,
};
use super::super::check;

#[test]
fn inventories_when_local_policy_root_keeps_full_managed_baseline() {
    let tree = library_workspace_root_tree(build_clippy_toml("library", false, true, "", ""));
    let facts = collected_facts(&tree);
    let mut results = Vec::new();

    check(
        &config_input(&facts, "apps/libsite/clippy.toml"),
        &mut results,
    );

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-CLIPPY-13");
    assert!(result.inventory, "expected inventory result: {results:#?}");
    assert_eq!(result.severity, Severity::Info);
    assert_eq!(result.title, "local clippy policy root is self-contained");
    assert_eq!(result.file.as_deref(), Some("apps/libsite/clippy.toml"));
    assert_eq!(
        result.message,
        "`apps/libsite/clippy.toml` contains the full managed clippy baseline for its subtree."
    );
}

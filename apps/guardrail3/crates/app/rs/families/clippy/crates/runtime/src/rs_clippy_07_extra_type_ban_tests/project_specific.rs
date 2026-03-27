use guardrail3_domain_report::Severity;

use super::super::super::test_support::{
    canonical_clippy_toml, collected_facts, config_input, prepend_ban_path, root_workspace_tree,
};
use super::super::check;

#[test]
fn inventories_project_specific_extra_type_bans() {
    let clippy = prepend_ban_path(
        &canonical_clippy_toml(),
        "disallowed-types",
        "std::sync::Arc",
        "good enough reason text",
    );
    let tree = root_workspace_tree(clippy);
    let facts = collected_facts(&tree);
    let mut results = Vec::new();

    check(&config_input(&facts, "clippy.toml"), &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-CLIPPY-07");
    assert!(result.inventory);
    assert_eq!(result.severity, Severity::Info);
    assert_eq!(result.title, "extra type ban");
    assert_eq!(
        result.message,
        "Additional type ban `std::sync::Arc` beyond baseline."
    );
    assert_eq!(result.file.as_deref(), Some("clippy.toml"));
}

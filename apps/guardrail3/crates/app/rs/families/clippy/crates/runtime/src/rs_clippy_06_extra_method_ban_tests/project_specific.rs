use guardrail3_domain_report::Severity;

use super::super::super::test_support::{
    build_fixture_clippy_toml, collected_facts, config_input, prepend_ban_path, root_workspace_tree,
};
use super::super::check;

#[test]
fn inventories_project_specific_extra_method_bans() {
    let clippy = prepend_ban_path(
        &build_fixture_clippy_toml("service", false, true, "", ""),
        "disallowed-methods",
        "std::io::stdin",
        "good enough reason text",
    );
    let tree = root_workspace_tree(clippy);
    let facts = collected_facts(&tree);
    let mut results = Vec::new();

    check(&config_input(&facts, "clippy.toml"), &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-CLIPPY-06");
    assert!(result.inventory);
    assert_eq!(result.severity, Severity::Info);
    assert_eq!(result.title, "extra method ban");
    assert_eq!(
        result.message,
        "Additional method ban `std::io::stdin` beyond baseline."
    );
    assert_eq!(result.file.as_deref(), Some("clippy.toml"));
}

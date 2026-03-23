use crate::domain::modules::clippy::build_clippy_toml;
use crate::domain::report::Severity;

use super::super::super::test_support::{
    collected_facts, config_input, prepend_ban_path, published_library_package_root_tree,
};
use super::super::check;

#[test]
fn library_global_state_type_bans_are_not_extra_for_library_profile() {
    let tree =
        published_library_package_root_tree(build_clippy_toml("library", false, true, "", ""));
    let facts = collected_facts(&tree);
    let mut results = Vec::new();

    check(&config_input(&facts, "clippy.toml"), &mut results);

    assert!(
        results.is_empty(),
        "expected library baseline global-state types to be treated as managed, not extra: {results:#?}"
    );
}

#[test]
fn inventories_project_specific_extra_type_bans_on_top_of_library_profile() {
    let clippy = prepend_ban_path(
        &build_clippy_toml("library", false, true, "", ""),
        "disallowed-types",
        "std::sync::Arc",
        "good enough reason text",
    );
    let tree = published_library_package_root_tree(clippy);
    let facts = collected_facts(&tree);
    let mut results = Vec::new();

    check(&config_input(&facts, "clippy.toml"), &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-CLIPPY-07");
    assert_eq!(result.severity, Severity::Info);
    assert!(result.inventory);
    assert_eq!(
        result.message,
        "Additional type ban `std::sync::Arc` beyond baseline."
    );
}

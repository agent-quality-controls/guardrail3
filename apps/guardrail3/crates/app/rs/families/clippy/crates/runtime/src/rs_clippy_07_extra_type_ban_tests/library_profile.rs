use guardrail3_domain_modules::clippy::build_clippy_toml;
use guardrail3_app_rs_family_clippy_assertions::rs_clippy_07_extra_type_ban as assertions;
use test_support::{prepend_ban_path, published_library_package_root_tree};

use super::super::run_for_tests;

#[test]
fn library_global_state_type_bans_are_not_extra_for_library_profile() {
    let tree =
        published_library_package_root_tree(build_clippy_toml("library", false, true, "", ""));
    let results = run_for_tests(&tree, "clippy.toml");
    assertions::assert_no_results(&results);
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
    let results = run_for_tests(&tree, "clippy.toml");
    assertions::assert_project_specific(&results, "std::sync::Arc");
}

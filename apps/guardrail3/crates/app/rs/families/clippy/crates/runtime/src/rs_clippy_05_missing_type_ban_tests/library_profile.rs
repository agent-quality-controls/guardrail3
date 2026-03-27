use guardrail3_domain_modules::clippy::build_clippy_toml;
use guardrail3_app_rs_family_clippy_assertions::rs_clippy_05_missing_type_ban as assertions;
use test_support::{published_library_package_root_tree, remove_ban_path};

use super::super::run_for_tests;

#[test]
fn does_not_inventory_library_only_global_state_type_bans_under_base_missing_type_rule() {
    let tree =
        published_library_package_root_tree(build_clippy_toml("library", false, true, "", ""));
    let results = run_for_tests(&tree, "clippy.toml");
    assertions::assert_excludes_library_global_state(&results);
}

#[test]
fn does_not_double_report_library_global_state_gaps() {
    let mut clippy = build_clippy_toml("library", false, true, "", "");
    for path in ["std::sync::LazyLock", "once_cell::sync::OnceCell"] {
        clippy = remove_ban_path(&clippy, "disallowed-types", path);
    }

    let tree = published_library_package_root_tree(clippy);
    let results = run_for_tests(&tree, "clippy.toml");
    assertions::assert_excludes_library_global_state(&results);
}

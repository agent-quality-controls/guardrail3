use guardrail3_domain_modules::clippy::build_clippy_toml;
use guardrail3_app_rs_family_clippy_assertions::rs_clippy_05_missing_type_ban as assertions;
use test_support::{published_library_package_root_tree, remove_ban_path};

use super::super::run_for_tests;

#[test]
fn inventories_library_only_global_state_type_bans_when_library_profile_baseline_is_present() {
    let tree =
        published_library_package_root_tree(build_clippy_toml("library", false, true, "", ""));
    let results = run_for_tests(&tree, "clippy.toml");
    assertions::assert_library_global_state_inventory(&results);
}

#[test]
fn errors_when_library_profile_is_missing_global_state_type_bans() {
    let mut clippy = build_clippy_toml("library", false, true, "", "");
    for path in ["std::sync::LazyLock", "once_cell::sync::OnceCell"] {
        clippy = remove_ban_path(&clippy, "disallowed-types", path);
    }

    let tree = published_library_package_root_tree(clippy);
    let results = run_for_tests(&tree, "clippy.toml");
    assertions::assert_missing_messages(
        &results,
        &[
            "`std::sync::LazyLock` is not present in `disallowed-types`.",
            "`once_cell::sync::OnceCell` is not present in `disallowed-types`.",
        ],
    );
}

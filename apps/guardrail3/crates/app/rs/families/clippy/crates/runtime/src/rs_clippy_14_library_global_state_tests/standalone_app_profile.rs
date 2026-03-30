use guardrail3_app_rs_family_clippy_assertions::rs_clippy_14_library_global_state as assertions;
use test_support::{build_fixture_clippy_toml, dir_entry, project_tree, remove_ban_path};

use super::super::run_for_tests;

#[test]
fn standalone_app_root_uses_rust_apps_profile_policy() {
    let clippy = remove_ban_path(
        &build_fixture_clippy_toml("library", false, true, "", ""),
        "disallowed-types",
        "std::sync::OnceLock",
    );
    let tree = project_tree(
        vec![
            ("", dir_entry(&["apps"], &["guardrail3.toml"])),
            ("apps", dir_entry(&["libsite"], &[])),
            (
                "apps/libsite",
                dir_entry(&[], &["Cargo.toml", "clippy.toml"]),
            ),
        ],
        vec![
            (
                "guardrail3.toml",
                "[profile]\nname = \"service\"\n[rust.apps.libsite]\ntype = \"library\"\n"
                    .to_owned(),
            ),
            (
                "apps/libsite/Cargo.toml",
                "[workspace]\nmembers = []\n[package]\nname = \"libsite\"\n".to_owned(),
            ),
            ("apps/libsite/clippy.toml", clippy),
        ],
    );

    let results = run_for_tests(&tree, "apps/libsite/clippy.toml");
    assertions::assert_missing_messages(
        &results,
        &["Library profile must ban `std::sync::OnceLock` in `disallowed-types`."],
        "apps/libsite/clippy.toml",
    );
}

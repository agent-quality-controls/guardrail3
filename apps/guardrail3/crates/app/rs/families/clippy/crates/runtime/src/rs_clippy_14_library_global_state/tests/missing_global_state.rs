use guardrail3_app_rs_family_clippy_assertions::rs_clippy_14_library_global_state as assertions;
use test_support::{build_fixture_clippy_toml, library_workspace_root_tree};

use super::helpers::run_for_tests;

#[test]
fn errors_for_every_missing_library_global_state_type_ban() {
    let tree =
        library_workspace_root_tree(build_fixture_clippy_toml("service", false, true, "", ""));
    let results = run_for_tests(&tree, "apps/libsite/clippy.toml");
    assertions::assert_missing_messages(
        &results,
        &[
            "Library profile must ban `once_cell::sync::Lazy` in `disallowed-types`.",
            "Library profile must ban `once_cell::sync::OnceCell` in `disallowed-types`.",
            "Library profile must ban `std::sync::LazyLock` in `disallowed-types`.",
            "Library profile must ban `std::sync::OnceLock` in `disallowed-types`.",
        ],
        "apps/libsite/clippy.toml",
    );
}

use guardrail3_app_rs_family_clippy_assertions::rs_clippy_14_library_global_state as assertions;
use test_support::{build_fixture_clippy_toml, package_library_workspace_root_tree};

use super::helpers::run_for_tests;

#[test]
fn package_workspace_root_uses_rust_packages_library_profile() {
    let tree = package_library_workspace_root_tree(build_fixture_clippy_toml(
        "service", false, true, "", "",
    ));
    let results = run_for_tests(&tree, "packages/shared-types/clippy.toml");
    assertions::assert_missing_messages(
        &results,
        &[
            "Library profile must ban `std::sync::LazyLock` in `disallowed-types`. Add it to `disallowed-types` in clippy.toml.",
            "Library profile must ban `std::sync::OnceLock` in `disallowed-types`. Add it to `disallowed-types` in clippy.toml.",
            "Library profile must ban `once_cell::sync::Lazy` in `disallowed-types`. Add it to `disallowed-types` in clippy.toml.",
            "Library profile must ban `once_cell::sync::OnceCell` in `disallowed-types`. Add it to `disallowed-types` in clippy.toml.",
        ],
        "packages/shared-types/clippy.toml",
    );
}

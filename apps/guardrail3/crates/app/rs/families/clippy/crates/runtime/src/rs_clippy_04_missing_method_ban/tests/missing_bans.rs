use guardrail3_app_rs_family_clippy_assertions::rs_clippy_04_missing_method_ban as assertions;
use test_support::{build_fixture_clippy_toml, remove_ban_path, root_workspace_tree};

use super::helpers::run_for_tests;

#[test]
fn errors_for_each_missing_required_method_ban() {
    let mut clippy = build_fixture_clippy_toml("service", false, true, "", "");
    for path in ["std::env::var", "std::process::abort"] {
        clippy = remove_ban_path(&clippy, "disallowed-methods", path);
    }

    let tree = root_workspace_tree(clippy);
    let results = run_for_tests(&tree, "clippy.toml");
    assertions::assert_missing_messages(
        &results,
        &[
            "`std::env::var` is not present in `disallowed-methods`. Add it to `disallowed-methods` in clippy.toml.",
            "`std::process::abort` is not present in `disallowed-methods`. Add it to `disallowed-methods` in clippy.toml.",
        ],
    );
}

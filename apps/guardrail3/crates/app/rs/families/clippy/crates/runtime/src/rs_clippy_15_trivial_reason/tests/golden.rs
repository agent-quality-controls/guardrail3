use test_support::{build_fixture_clippy_toml, root_workspace_tree};

use super::helpers::run_for_tests;

#[test]
fn stays_quiet_for_canonical_ban_entries() {
    let tree = root_workspace_tree(build_fixture_clippy_toml("service", false, true, "", ""));
    let results = run_for_tests(&tree, "clippy.toml");
    assert!(results.is_empty(), "ban-only surfaces should stay quiet: {results:#?}");
}

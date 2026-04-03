use guardrail3_app_rs_family_clippy_assertions::rs_clippy_04_missing_method_ban as assertions;
use test_support::{build_fixture_clippy_toml, garde_disabled_root_tree, remove_ban_path};

use super::helpers::run_for_tests;

#[test]
fn drops_garde_owned_method_requirements_when_garde_is_disabled() {
    let expected = assertions::expected_garde_disabled_method_bans();
    let garde_method_bans = assertions::managed_method_bans()
        .iter()
        .filter(|path| !expected.contains(path))
        .copied()
        .collect::<Vec<_>>();
    let mut clippy = build_fixture_clippy_toml("service", false, true, "", "");
    for path in &garde_method_bans {
        clippy = remove_ban_path(&clippy, "disallowed-methods", path);
    }

    let tree = garde_disabled_root_tree(clippy);
    let results = run_for_tests(&tree, "clippy.toml");
    assertions::assert_garde_disabled(&results, &expected, "clippy.toml");
}

use guardrail3_app_rs_family_clippy_assertions::rs_clippy_05_missing_type_ban as assertions;
use test_support::{build_fixture_clippy_toml, garde_disabled_root_tree, remove_ban_path};

use super::super::run_for_tests;

#[test]
fn drops_garde_owned_type_requirements_when_garde_is_disabled() {
    let garde_type_bans = crate::clippy_support::expected_required_type_bans(true)
        .into_iter()
        .filter(|path| !crate::clippy_support::expected_required_type_bans(false).contains(path))
        .collect::<Vec<_>>();
    let mut clippy = build_fixture_clippy_toml("service", false, true, "", "");
    for path in &garde_type_bans {
        clippy = remove_ban_path(&clippy, "disallowed-types", path);
    }

    let tree = garde_disabled_root_tree(clippy);
    let results = run_for_tests(&tree, "clippy.toml");
    let expected = crate::clippy_support::expected_required_type_bans(false);
    assertions::assert_garde_disabled(&results, &expected, "clippy.toml");
}

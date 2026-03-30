use guardrail3_app_rs_family_clippy_assertions::rs_clippy_01_coverage as assertions;
use test_support::{build_fixture_clippy_toml, write_file};

use super::super::{copy_fixture_for_tests, run_for_tests};

#[test]
fn errors_only_for_the_top_workspace_root_when_nested_workspaces_exist() {
    let tmp = copy_fixture_for_tests();
    write_file(
        tmp.path(),
        "apps/devctl/clippy.toml",
        &build_fixture_clippy_toml("service", false, true, "", ""),
    );
    write_file(
        tmp.path(),
        "packages/shared-types/clippy.toml",
        &build_fixture_clippy_toml("library", false, true, "", ""),
    );

    let results = run_for_tests(tmp.path());
    assertions::assert_selective_uncovered(
        &results,
        &["workspace root is not covered by any allowed clippy.toml at a workspace root."],
        &[""],
    );
}

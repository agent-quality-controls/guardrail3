use guardrail3_app_rs_family_clippy_assertions::rs_clippy_01_coverage as assertions;
use test_support::{build_fixture_clippy_toml, write_file};

use super::super::{copy_fixture_for_tests, run_with_validation_scope_for_tests};

#[test]
fn ignores_nested_workspace_policy_roots_when_validation_scope_targets_one_app() {
    let tmp = copy_fixture_for_tests();
    write_file(
        tmp.path(),
        "clippy.toml",
        &build_fixture_clippy_toml("service", false, true, "", ""),
    );
    write_file(
        tmp.path(),
        "apps/devctl/clippy.toml",
        &build_fixture_clippy_toml("service", false, true, "", ""),
    );

    let results = run_with_validation_scope_for_tests(tmp.path(), "apps/backend");
    assertions::assert_multi_root_coverage(
        &results,
        &[
            (
                "workspace root is covered by `clippy.toml`.",
                assertions::Severity::Info,
                true,
                Some("clippy.toml"),
                "Rust unit covered by clippy.toml",
            ),
        ],
    );
}

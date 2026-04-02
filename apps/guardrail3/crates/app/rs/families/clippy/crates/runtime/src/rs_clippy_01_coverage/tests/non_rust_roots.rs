use guardrail3_app_rs_family_clippy_assertions::rs_clippy_01_coverage as assertions;
use test_support::{build_fixture_clippy_toml, write_file};

use super::super::{copy_fixture_for_tests, run_for_tests};

#[test]
fn ignores_non_rust_roots_in_the_multi_root_fixture() {
    let tmp = copy_fixture_for_tests();
    write_file(
        tmp.path(),
        "clippy.toml",
        &build_fixture_clippy_toml("service", false, true, "", ""),
    );

    let results = run_for_tests(tmp.path());
    assertions::assert_excludes_non_rust_roots(&results);
}

use guardrail3_app_rs_family_clippy_assertions::rs_clippy_01_coverage as assertions;
use test_support::{create_dir_all, create_temp_dir, write_file};

use super::super::run_for_tests;

#[test]
fn ignores_uncovered_non_workspace_package_roots() {
    let tmp = create_temp_dir("rs-clippy-01-standalone-uncovered");
    create_dir_all(&tmp.path().join("packages/shared-types"));
    write_file(
        tmp.path(),
        "packages/shared-types/Cargo.toml",
        "[package]\nname = \"shared-types\"\n",
    );

    let results = run_for_tests(tmp.path());
    assertions::assert_multi_root_coverage(&results, &[]);
}

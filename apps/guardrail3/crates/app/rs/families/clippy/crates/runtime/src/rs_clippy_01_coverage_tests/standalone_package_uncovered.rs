use guardrail3_app_rs_family_clippy_assertions::rs_clippy_01_coverage as assertions;
use test_support::{create_temp_dir, write_file};

use super::super::run_for_tests;

#[test]
fn reports_uncovered_legal_workspace_root_without_clippy_config() {
    let tmp = create_temp_dir("rs-clippy-01-uncovered-workspace");
    write_file(
        tmp.path(),
        "Cargo.toml",
        "[workspace]\nmembers = []\n",
    );

    let results = run_for_tests(tmp.path());
    assertions::assert_selective_uncovered(
        &results,
        &["workspace root is not covered by any allowed clippy.toml at a workspace root."],
        &[""],
    );
}

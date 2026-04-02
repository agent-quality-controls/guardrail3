use guardrail3_app_rs_family_clippy_assertions::rs_clippy_01_coverage as assertions;
use test_support::{create_dir_all, create_temp_dir, write_file};

use super::super::run_for_tests;

#[test]
fn errors_for_an_uncovered_legal_workspace_root() {
    let tmp = create_temp_dir("rs-clippy-01-uncovered");
    create_dir_all(&tmp.path().join("apps/libsite"));
    write_file(
        tmp.path(),
        "apps/libsite/Cargo.toml",
        "[workspace]\nmembers = []\n",
    );

    let results = run_for_tests(tmp.path());
    assertions::assert_selective_uncovered(
        &results,
        &[
            "workspace root `apps/libsite` is not covered by any allowed clippy.toml at a workspace root.",
        ],
        &["apps/libsite"],
    );
}

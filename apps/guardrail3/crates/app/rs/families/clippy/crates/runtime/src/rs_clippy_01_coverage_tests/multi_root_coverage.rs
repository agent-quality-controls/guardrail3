use guardrail3_app_rs_family_clippy_assertions::rs_clippy_01_coverage as assertions;
use test_support::{create_dir_all, create_temp_dir, write_file};

use super::super::run_for_tests;

#[test]
fn inventories_workspace_local_root_clippy_config() {
    let tmp = create_temp_dir("rs-clippy-01-workspace-local");
    create_dir_all(&tmp.path().join("apps/libsite"));
    write_file(
        tmp.path(),
        "apps/libsite/Cargo.toml",
        "[workspace]\nmembers = []\n",
    );
    write_file(tmp.path(), "apps/libsite/clippy.toml", "msrv = \"1.85\"\n");

    let results = run_for_tests(tmp.path());
    assertions::assert_multi_root_coverage(
        &results,
        &[(
            "workspace root `apps/libsite` is covered by `apps/libsite/clippy.toml`.",
            assertions::Severity::Info,
            true,
            Some("apps/libsite/clippy.toml"),
            "Rust unit covered by clippy.toml",
        )],
    );
}

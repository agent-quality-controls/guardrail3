use guardrail3_app_rs_family_clippy_assertions::rs_clippy_01_coverage as assertions;
use test_support::{create_temp_dir, write_file};

use super::super::run_for_tests;

#[test]
fn inventories_legal_workspace_root_clippy_config() {
    let tmp = create_temp_dir("rs-clippy-01-legal-workspace");
    write_file(tmp.path(), "Cargo.toml", "[workspace]\nmembers = []\n");
    write_file(tmp.path(), "clippy.toml", "msrv = \"1.85\"\n");

    let results = run_for_tests(tmp.path());
    assertions::assert_multi_root_coverage(
        &results,
        &[(
            "workspace root is covered by `clippy.toml`.",
            assertions::Severity::Info,
            true,
            Some("clippy.toml"),
            "Rust unit covered by clippy.toml",
        )],
    );
}

use guardrail3_app_rs_family_clippy_assertions::rs_clippy_01_coverage as assertions;
use test_support::{build_fixture_clippy_toml, create_dir_all, create_temp_dir, write_file};

use super::super::run_for_tests;

#[test]
fn inventories_local_clippy_coverage_for_a_standalone_package_root() {
    let tmp = create_temp_dir("rs-clippy-01-standalone-package");
    create_dir_all(&tmp.path().join("tools/helper"));
    write_file(
        tmp.path(),
        "tools/helper/Cargo.toml",
        "[package]\nname = \"helper\"\nversion = \"0.1.0\"\n",
    );
    write_file(
        tmp.path(),
        "tools/helper/clippy.toml",
        &build_fixture_clippy_toml("service", false, true, "", ""),
    );

    let results = run_for_tests(tmp.path());
    assertions::assert_multi_root_coverage(
        &results,
        &[(
            "standalone package root `tools/helper` is covered by `tools/helper/clippy.toml`.",
            assertions::Severity::Info,
            true,
            Some("tools/helper/clippy.toml"),
            "Rust unit covered by clippy.toml",
        )],
    );
}

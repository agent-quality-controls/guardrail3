use guardrail3_app_rs_family_clippy_assertions::rs_clippy_01_coverage as assertions;
use test_support::{build_fixture_clippy_toml, create_dir_all, create_temp_dir, write_file};

use super::super::run_for_tests;

#[test]
fn errors_when_a_routed_cargo_root_cannot_be_parsed() {
    let tmp = create_temp_dir("rs-clippy-01-unparseable-routed-cargo");
    create_dir_all(&tmp.path().join("apps/backend/crates/core"));
    write_file(
        tmp.path(),
        "apps/backend/Cargo.toml",
        "[workspace\nmembers = [\"crates/*\"]\n",
    );
    write_file(
        tmp.path(),
        "apps/backend/clippy.toml",
        &build_fixture_clippy_toml("service", false, true, "", ""),
    );
    write_file(
        tmp.path(),
        "apps/backend/crates/core/Cargo.toml",
        "[package]\nname = \"core\"\n",
    );

    let results = run_for_tests(tmp.path());
    assertions::assert_unparseable_routed_cargo_root(&results, "apps/backend/Cargo.toml");
}

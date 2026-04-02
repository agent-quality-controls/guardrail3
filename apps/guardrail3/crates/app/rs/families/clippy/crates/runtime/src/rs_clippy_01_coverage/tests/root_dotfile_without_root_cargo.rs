use guardrail3_app_rs_family_clippy_assertions::rs_clippy_01_coverage as assertions;
use test_support::{build_fixture_clippy_toml, create_dir_all, create_temp_dir, write_file};

use super::super::run_for_tests;

#[test]
fn repo_root_dotfile_does_not_cover_descendant_workspace_without_root_cargo() {
    let tmp = create_temp_dir("root-dotfile-without-root-cargo");
    create_dir_all(&tmp.path().join("apps/backend/crates/core"));
    write_file(
        tmp.path(),
        ".clippy.toml",
        &build_fixture_clippy_toml("service", false, true, "", ""),
    );
    write_file(
        tmp.path(),
        "apps/backend/Cargo.toml",
        "[workspace]\nmembers = [\"crates/*\"]\n",
    );
    write_file(
        tmp.path(),
        "apps/backend/crates/core/Cargo.toml",
        "[package]\nname = \"core\"\n",
    );

    let results = run_for_tests(tmp.path());
    assertions::assert_multi_root_coverage(
        &results,
        &[(
            "workspace root `apps/backend` is not covered by any allowed clippy.toml at a workspace root.",
            assertions::Severity::Error,
            false,
            Some("apps/backend"),
            "Rust unit uncovered by clippy.toml",
        )],
    );
}

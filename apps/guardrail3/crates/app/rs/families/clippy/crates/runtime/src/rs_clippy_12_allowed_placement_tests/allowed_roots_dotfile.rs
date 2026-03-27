use guardrail3_app_rs_family_clippy_assertions::rs_clippy_12_allowed_placement as assertions;
use test_support::{build_fixture_clippy_toml, dir_entry, project_tree};

use super::super::run_for_tests;

#[test]
fn allows_dotfile_at_validation_workspace_and_standalone_package_roots() {
    let tree = project_tree(
        vec![
            (
                "",
                dir_entry(&["apps", "packages"], &["Cargo.toml", ".clippy.toml"]),
            ),
            ("apps", dir_entry(&["backend"], &[])),
            (
                "apps/backend",
                dir_entry(&[], &["Cargo.toml", ".clippy.toml"]),
            ),
            ("packages", dir_entry(&["shared-types"], &[])),
            (
                "packages/shared-types",
                dir_entry(&[], &["Cargo.toml", ".clippy.toml"]),
            ),
        ],
        vec![
            ("Cargo.toml", "[workspace]\nmembers = []".to_owned()),
            (
                ".clippy.toml",
                build_fixture_clippy_toml("service", false, true, "", ""),
            ),
            (
                "apps/backend/Cargo.toml",
                "[workspace]\nmembers = []".to_owned(),
            ),
            (
                "apps/backend/.clippy.toml",
                build_fixture_clippy_toml("service", false, true, "", ""),
            ),
            (
                "packages/shared-types/Cargo.toml",
                "[package]\nname = \"shared-types\"\n".to_owned(),
            ),
            (
                "packages/shared-types/.clippy.toml",
                build_fixture_clippy_toml("service", false, true, "", ""),
            ),
        ],
    );

    let results = run_for_tests(&tree);
    assertions::assert_no_results(&results);
}

use guardrail3_app_rs_family_clippy_assertions::rs_clippy_12_allowed_placement as assertions;
use test_support::{build_fixture_clippy_toml, dir_entry, project_tree};

use super::super::run_for_tests;

#[test]
fn inventories_dotfile_only_at_the_top_workspace_root_when_nested_workspaces_exist() {
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
                "[workspace]\nmembers = []\n".to_owned(),
            ),
            (
                "packages/shared-types/.clippy.toml",
                build_fixture_clippy_toml("service", false, true, "", ""),
            ),
        ],
    );

    let results = run_for_tests(&tree);
    assertions::assert_allowed_files(&results, &[".clippy.toml"]);
}

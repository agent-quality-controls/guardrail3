use guardrail3_app_rs_family_clippy_assertions::rs_clippy_12_allowed_placement as assertions;
use test_support::{build_fixture_clippy_toml, dir_entry, project_tree};

use super::super::run_with_validation_scope_for_tests;

#[test]
fn rejects_nested_workspace_policy_roots_when_validation_scope_targets_one_app() {
    let tree = project_tree(
        vec![
            ("", dir_entry(&["apps"], &["Cargo.toml", "clippy.toml"])),
            ("apps", dir_entry(&["backend", "devctl"], &[])),
            (
                "apps/backend",
                dir_entry(&["crates"], &["Cargo.toml", "clippy.toml"]),
            ),
            ("apps/backend/crates", dir_entry(&["core"], &[])),
            ("apps/backend/crates/core", dir_entry(&[], &["Cargo.toml"])),
            (
                "apps/devctl",
                dir_entry(&["crates"], &["Cargo.toml", "clippy.toml"]),
            ),
            ("apps/devctl/crates", dir_entry(&["cli"], &[])),
            (
                "apps/devctl/crates/cli",
                dir_entry(&[], &["Cargo.toml", "clippy.toml"]),
            ),
        ],
        vec![
            ("Cargo.toml", "[workspace]\nmembers = []".to_owned()),
            (
                "clippy.toml",
                build_fixture_clippy_toml("service", false, true, "", ""),
            ),
            (
                "apps/backend/Cargo.toml",
                "[workspace]\nmembers = [\"crates/*\"]\n".to_owned(),
            ),
            (
                "apps/backend/crates/core/Cargo.toml",
                "[package]\nname = \"core\"\n".to_owned(),
            ),
            (
                "apps/backend/clippy.toml",
                build_fixture_clippy_toml("service", false, true, "", ""),
            ),
            (
                "apps/devctl/Cargo.toml",
                "[workspace]\nmembers = [\"crates/*\"]\n".to_owned(),
            ),
            (
                "apps/devctl/crates/cli/Cargo.toml",
                "[package]\nname = \"cli\"\n".to_owned(),
            ),
            (
                "apps/devctl/crates/cli/clippy.toml",
                build_fixture_clippy_toml("service", false, true, "", ""),
            ),
            (
                "apps/devctl/clippy.toml",
                build_fixture_clippy_toml("service", false, true, "", ""),
            ),
        ],
    );

    let results = run_with_validation_scope_for_tests(&tree, "apps/backend");
    assertions::assert_allowed_files(&results, &["clippy.toml"]);
}

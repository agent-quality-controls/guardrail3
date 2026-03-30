use guardrail3_app_rs_family_clippy_assertions::rs_clippy_12_allowed_placement as assertions;
use test_support::{dir_entry, project_tree};

use super::super::run_for_tests;

#[test]
fn rejects_lower_precedence_same_root_sibling_config_at_workspace_roots() {
    let tree = project_tree(
        vec![
            ("", dir_entry(&["apps"], &[])),
            ("apps", dir_entry(&["backend"], &[])),
            (
                "apps/backend",
                dir_entry(&["crates"], &["Cargo.toml", "clippy.toml", ".clippy.toml"]),
            ),
            ("apps/backend/crates", dir_entry(&["core"], &[])),
            ("apps/backend/crates/core", dir_entry(&[], &["Cargo.toml"])),
        ],
        vec![
            (
                "apps/backend/Cargo.toml",
                "[workspace]\nmembers = [\"crates/*\"]".to_owned(),
            ),
            (
                "apps/backend/crates/core/Cargo.toml",
                "[package]\nname = \"core\"".to_owned(),
            ),
            (
                "apps/backend/clippy.toml",
                "max-struct-bools = 3".to_owned(),
            ),
            (
                "apps/backend/.clippy.toml",
                "max-struct-bools = 4".to_owned(),
            ),
        ],
    );

    let results = run_for_tests(&tree);
    assertions::assert_allowed_files(&results, &["apps/backend/.clippy.toml"]);
    assertions::assert_same_root_conflict(
        &results,
        "apps/backend/clippy.toml",
        "apps/backend/.clippy.toml",
    );
}

#[test]
fn rejects_lower_precedence_same_root_sibling_config_at_standalone_package_roots() {
    let tree = project_tree(
        vec![
            ("", dir_entry(&["packages"], &[])),
            ("packages", dir_entry(&["cli"], &[])),
            (
                "packages/cli",
                dir_entry(&[], &["Cargo.toml", "clippy.toml", ".clippy.toml"]),
            ),
        ],
        vec![
            (
                "packages/cli/Cargo.toml",
                "[package]\nname = \"cli\"".to_owned(),
            ),
            (
                "packages/cli/clippy.toml",
                "max-struct-bools = 3".to_owned(),
            ),
            (
                "packages/cli/.clippy.toml",
                "max-struct-bools = 4".to_owned(),
            ),
        ],
    );

    let results = run_for_tests(&tree);
    assertions::assert_allowed_files(&results, &["packages/cli/.clippy.toml"]);
    assertions::assert_same_root_conflict(
        &results,
        "packages/cli/clippy.toml",
        "packages/cli/.clippy.toml",
    );
}

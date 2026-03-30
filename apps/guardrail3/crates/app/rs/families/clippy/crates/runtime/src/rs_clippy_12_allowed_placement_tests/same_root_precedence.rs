use guardrail3_app_rs_family_clippy_assertions::rs_clippy_12_allowed_placement as assertions;
use test_support::{dir_entry, project_tree, same_root_dual_config_tree};

use super::super::run_for_tests;

#[test]
fn rejects_lower_precedence_same_root_sibling_config() {
    let results = run_for_tests(&same_root_dual_config_tree());
    assertions::assert_allowed_files(&results, &[".clippy.toml"]);
    assertions::assert_same_root_conflict(&results, "clippy.toml", ".clippy.toml");
}

#[test]
fn rejects_lower_precedence_same_root_sibling_config_at_a_workspace_root() {
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
                "[package]\nname = \"core\"\n".to_owned(),
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
fn forbids_both_same_root_sibling_configs_at_a_non_workspace_root() {
    let tree = project_tree(
        vec![
            ("", dir_entry(&["tools"], &[])),
            ("tools", dir_entry(&["helper"], &[])),
            (
                "tools/helper",
                dir_entry(&[], &["Cargo.toml", "clippy.toml", ".clippy.toml"]),
            ),
        ],
        vec![
            (
                "tools/helper/Cargo.toml",
                "[package]\nname = \"helper\"\nversion = \"0.1.0\"\n".to_owned(),
            ),
            (
                "tools/helper/clippy.toml",
                "max-struct-bools = 3".to_owned(),
            ),
            (
                "tools/helper/.clippy.toml",
                "max-struct-bools = 4".to_owned(),
            ),
        ],
    );

    let results = run_for_tests(&tree);
    assertions::assert_allowed_files(&results, &[]);
    assertions::assert_forbidden_files(
        &results,
        &["tools/helper/.clippy.toml", "tools/helper/clippy.toml"],
    );
}

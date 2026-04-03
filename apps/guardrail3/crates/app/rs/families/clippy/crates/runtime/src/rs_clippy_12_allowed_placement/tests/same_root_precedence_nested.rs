use guardrail3_app_rs_family_clippy_assertions::rs_clippy_12_allowed_placement as assertions;
use test_support::{dir_entry, project_tree};

use super::helpers::run_for_tests;

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
fn rejects_lower_precedence_same_root_sibling_config_at_a_legal_workspace_root() {
    let tree = project_tree(
        vec![
            ("", dir_entry(&["apps"], &["guardrail3.toml"])),
            ("apps", dir_entry(&["libsite"], &[])),
            (
                "apps/libsite",
                dir_entry(&[], &["Cargo.toml", "clippy.toml", ".clippy.toml"]),
            ),
        ],
        vec![
            (
                "guardrail3.toml",
                "[profile]\nname = \"service\"\n[rust.apps.libsite]\ntype = \"library\"\n"
                    .to_owned(),
            ),
            (
                "apps/libsite/Cargo.toml",
                "[workspace]\nmembers = []\n".to_owned(),
            ),
            (
                "apps/libsite/clippy.toml",
                "max-struct-bools = 3".to_owned(),
            ),
            (
                "apps/libsite/.clippy.toml",
                "max-struct-bools = 4".to_owned(),
            ),
        ],
    );

    let results = run_for_tests(&tree);
    assertions::assert_allowed_files(&results, &["apps/libsite/.clippy.toml"]);
    assertions::assert_same_root_conflict(
        &results,
        "apps/libsite/clippy.toml",
        "apps/libsite/.clippy.toml",
    );
}

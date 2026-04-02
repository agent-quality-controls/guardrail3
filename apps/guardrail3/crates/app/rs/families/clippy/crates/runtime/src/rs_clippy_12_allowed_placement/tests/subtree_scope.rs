use guardrail3_app_rs_family_clippy_assertions::rs_clippy_12_allowed_placement as assertions;
use test_support::{dir_entry, project_tree};

use super::super::run_with_validation_scope_for_tests;

#[test]
fn respects_validation_scope_across_sibling_legal_workspaces() {
    let tree = project_tree(
        vec![
            ("", dir_entry(&["apps"], &[])),
            ("apps", dir_entry(&["backend", "devctl"], &[])),
            (
                "apps/backend",
                dir_entry(&[], &["Cargo.toml", "clippy.toml"]),
            ),
            (
                "apps/devctl",
                dir_entry(&[], &["Cargo.toml", "clippy.toml"]),
            ),
        ],
        vec![
            (
                "apps/backend/Cargo.toml",
                "[workspace]\nmembers = []\n".to_owned(),
            ),
            ("apps/backend/clippy.toml", "msrv = \"1.85\"\n".to_owned()),
            (
                "apps/devctl/Cargo.toml",
                "[workspace]\nmembers = []\n".to_owned(),
            ),
            ("apps/devctl/clippy.toml", "msrv = \"1.85\"\n".to_owned()),
        ],
    );

    let results = run_with_validation_scope_for_tests(&tree, "apps/backend");
    assertions::assert_allowed_files(&results, &["apps/backend/clippy.toml"]);
}

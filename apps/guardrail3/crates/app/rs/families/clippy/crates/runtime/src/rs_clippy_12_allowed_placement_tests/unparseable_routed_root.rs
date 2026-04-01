use test_support::{dir_entry, project_tree};

use super::super::run_for_tests;

#[test]
fn ignores_configs_attached_to_malformed_workspace_roots_because_shared_legality_owns_root_legality() {
    let tree = project_tree(
        vec![
            ("", dir_entry(&["apps"], &[])),
            ("apps", dir_entry(&["backend"], &[])),
            (
                "apps/backend",
                dir_entry(&[], &["Cargo.toml", "clippy.toml"]),
            ),
        ],
        vec![
            (
                "apps/backend/Cargo.toml",
                "[workspace\nmembers = [\"crates/*\"]".to_owned(),
            ),
            (
                "apps/backend/clippy.toml",
                "max-struct-bools = 3\n".to_owned(),
            ),
        ],
    );

    let results = run_for_tests(&tree);
    assert!(
        results
            .iter()
            .all(|result| result.file() != Some("apps/backend/clippy.toml")),
        "clippy should ignore configs attached to malformed workspace roots because shared legality filters them before clippy runs: {results:#?}"
    );
}

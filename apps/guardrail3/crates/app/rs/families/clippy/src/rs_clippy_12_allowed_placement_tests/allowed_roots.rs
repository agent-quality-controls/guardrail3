use super::super::super::test_support::{
    canonical_clippy_toml, collected_facts, dir_entry, project_tree,
};

#[test]
fn allows_validation_workspace_and_standalone_package_roots() {
    let tree = project_tree(
        vec![
            (
                "",
                dir_entry(&["apps", "packages"], &["Cargo.toml", "clippy.toml"]),
            ),
            ("apps", dir_entry(&["backend"], &[])),
            (
                "apps/backend",
                dir_entry(&[], &["Cargo.toml", "clippy.toml"]),
            ),
            ("packages", dir_entry(&["shared-types"], &[])),
            (
                "packages/shared-types",
                dir_entry(&[], &["Cargo.toml", "clippy.toml"]),
            ),
        ],
        vec![
            ("Cargo.toml", "[workspace]\nmembers = []".to_owned()),
            ("clippy.toml", canonical_clippy_toml()),
            (
                "apps/backend/Cargo.toml",
                "[workspace]\nmembers = []".to_owned(),
            ),
            ("apps/backend/clippy.toml", canonical_clippy_toml()),
            (
                "packages/shared-types/Cargo.toml",
                "[package]\nname = \"shared-types\"\n".to_owned(),
            ),
            ("packages/shared-types/clippy.toml", canonical_clippy_toml()),
        ],
    );

    let facts = collected_facts(&tree);

    assert!(
        facts.forbidden_configs.is_empty(),
        "expected allowed policy roots to stay out of RS-CLIPPY-12: {:#?}",
        facts.forbidden_configs
    );
}

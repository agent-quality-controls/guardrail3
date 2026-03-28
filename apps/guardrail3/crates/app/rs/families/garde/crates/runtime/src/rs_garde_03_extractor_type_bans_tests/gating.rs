use test_support::{dir_entry, project_tree, temp_root};

#[test]
fn ignores_workspace_when_garde_missing() {
    let root = temp_root("gating-garde-03");
    let tree = project_tree(
        vec![("", dir_entry(&[], &["Cargo.toml", "clippy.toml"]))],
        vec![
            (
                "Cargo.toml",
                "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n",
            ),
            ("clippy.toml", "disallowed-types = []"),
        ],
        root.clone(),
    );
    let results = crate::test_fixtures::run_family(&tree);
    let filtered: Vec<_> = results
        .into_iter()
        .filter(|r| r.id == "RS-GARDE-03")
        .collect();
    assert!(
        filtered.is_empty(),
        "Expected no RS-GARDE-03 results when garde is missing"
    );

    std::fs::remove_dir_all(&root).expect("remove temp root");
}

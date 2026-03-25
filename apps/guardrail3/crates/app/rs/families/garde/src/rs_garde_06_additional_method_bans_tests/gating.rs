use crate::check;
use crate::test_support::{dir_entry, project_tree, temp_root};

#[test]
fn ignores_workspace_when_garde_missing() {
    let root = temp_root("gating-garde-06");
    let tree = project_tree(
        vec![("", dir_entry(&[], &["Cargo.toml", "clippy.toml"]))],
        vec![
            (
                "Cargo.toml",
                "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n",
            ),
            ("clippy.toml", "disallowed-methods = []"),
        ],
        root.clone(),
    );
    let results = check(&tree, None);
    let filtered: Vec<_> = results
        .into_iter()
        .filter(|r| r.id == "RS-GARDE-06")
        .collect();
    assert!(
        filtered.is_empty(),
        "Expected no RS-GARDE-06 results when garde is missing"
    );

    std::fs::remove_dir_all(&root).expect("remove temp root");
}

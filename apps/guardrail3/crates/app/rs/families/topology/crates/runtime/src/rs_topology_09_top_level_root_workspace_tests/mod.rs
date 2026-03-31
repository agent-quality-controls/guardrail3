use test_support::{entry, tree};

#[test]
fn top_level_governed_root_must_be_workspace() {
    let tree = tree(
        &[
            ("", entry(&["apps"], &[])),
            ("apps", entry(&["backend"], &[])),
            ("apps/backend", entry(&[], &["Cargo.toml"])),
        ],
        &[("apps/backend/Cargo.toml", "[package]\nname = \"backend\"\n")],
    );

    let results = crate::check_test_tree(&tree);
    let result = results
        .iter()
        .find(|result| result.id() == "RS-TOPOLOGY-09")
        .expect("expected RS-TOPOLOGY-09 result");

    assert_eq!(result.file(), Some("apps/backend/Cargo.toml"));
}

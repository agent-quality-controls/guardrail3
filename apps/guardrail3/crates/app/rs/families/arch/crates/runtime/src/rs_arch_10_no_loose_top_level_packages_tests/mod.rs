use test_support::{entry, tree};

#[test]
fn loose_top_level_package_is_reported() {
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
        .find(|result| result.id() == "RS-ARCH-10")
        .expect("expected RS-ARCH-10 result");

    assert_eq!(result.file(), Some("apps/backend/Cargo.toml"));
}

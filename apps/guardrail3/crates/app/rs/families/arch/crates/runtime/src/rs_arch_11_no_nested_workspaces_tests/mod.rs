use test_support::{entry, tree};

#[test]
fn nested_workspace_is_reported() {
    let tree = tree(
        &[
            ("", entry(&["apps"], &[])),
            ("apps", entry(&["backend"], &[])),
            ("apps/backend", entry(&["crates"], &["Cargo.toml"])),
            ("apps/backend/crates", entry(&["api"], &[])),
            ("apps/backend/crates/api", entry(&[], &["Cargo.toml"])),
        ],
        &[
            (
                "apps/backend/Cargo.toml",
                "[workspace]\nmembers = [\"crates/api\"]\nresolver = \"2\"\n",
            ),
            (
                "apps/backend/crates/api/Cargo.toml",
                "[workspace]\nmembers = []\nresolver = \"2\"\n",
            ),
        ],
    );

    let results = crate::check_test_tree(&tree);
    let result = results
        .iter()
        .find(|result| result.id() == "RS-ARCH-11")
        .expect("expected RS-ARCH-11 result");

    assert_eq!(result.file(), Some("apps/backend/crates/api/Cargo.toml"));
}

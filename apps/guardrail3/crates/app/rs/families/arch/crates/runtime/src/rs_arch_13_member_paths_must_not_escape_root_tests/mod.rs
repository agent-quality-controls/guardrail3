use test_support::{entry, tree};

#[test]
fn escaping_member_pattern_is_reported() {
    let tree = tree(
        &[
            ("", entry(&["apps"], &[])),
            ("apps", entry(&["backend"], &[])),
            ("apps/backend", entry(&[], &["Cargo.toml"])),
        ],
        &[(
            "apps/backend/Cargo.toml",
            "[workspace]\nmembers = [\"../shared\"]\nresolver = \"2\"\n",
        )],
    );

    let results = crate::check_test_tree(&tree);
    let result = results
        .iter()
        .find(|result| result.id() == "RS-ARCH-13")
        .expect("expected RS-ARCH-13 result");

    assert_eq!(result.file(), Some("apps/backend/Cargo.toml"));
}

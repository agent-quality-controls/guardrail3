use super::super::super::test_support::{
    APP_WORKSPACE_CARGO, PACKAGE_CARGO, check_results, entry, error_results, tree,
};

#[test]
fn golden_layout_has_no_classification_errors() {
    let results = check_results(&tree(
        &[
            ("", entry(&["apps", "packages"], &[])),
            ("apps", entry(&["backend"], &[])),
            ("apps/backend", entry(&[], &["Cargo.toml"])),
            ("packages", entry(&["shared"], &[])),
            ("packages/shared", entry(&[], &["Cargo.toml"])),
        ],
        &[
            ("apps/backend/Cargo.toml", APP_WORKSPACE_CARGO),
            ("packages/shared/Cargo.toml", PACKAGE_CARGO),
        ],
    ));

    assert!(
        error_results(&results, "RS-ARCH-01").is_empty(),
        "unexpected classification errors: {results:#?}"
    );
}

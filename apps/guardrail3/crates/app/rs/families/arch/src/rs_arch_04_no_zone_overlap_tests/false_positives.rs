use super::super::super::test_support::{
    APP_WORKSPACE_CARGO, PACKAGE_CARGO, check_results, entry, error_results, tree,
};

#[test]
fn sibling_app_and_package_roots_do_not_overlap() {
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
        error_results(&results, "RS-ARCH-04").is_empty(),
        "sibling app/package roots should not overlap: {results:#?}"
    );
}

#[test]
fn ambiguous_roots_do_not_also_emit_zone_overlap_findings() {
    let results = check_results(&tree(
        &[
            ("", entry(&["apps"], &[])),
            ("apps", entry(&["backend"], &[])),
            ("apps/backend", entry(&["packages"], &["Cargo.toml"])),
            ("apps/backend/packages", entry(&["shared"], &[])),
            ("apps/backend/packages/shared", entry(&[], &["Cargo.toml"])),
        ],
        &[
            ("apps/backend/Cargo.toml", APP_WORKSPACE_CARGO),
            ("apps/backend/packages/shared/Cargo.toml", PACKAGE_CARGO),
        ],
    ));

    assert!(
        error_results(&results, "RS-ARCH-04").is_empty(),
        "ambiguous roots belong to RS-ARCH-01/03, not RS-ARCH-04 overlap reporting: {results:#?}"
    );
}

use super::super::super::test_support::{
    APP_WORKSPACE_CARGO, check_results, entry, error_results, tree,
};

#[test]
fn nested_app_inside_app_does_not_count_as_dual_family_ownership() {
    let results = check_results(&tree(
        &[
            ("", entry(&["apps"], &[])),
            ("apps", entry(&["backend"], &[])),
            ("apps/backend", entry(&["apps"], &["Cargo.toml"])),
            ("apps/backend/apps", entry(&["worker"], &[])),
            ("apps/backend/apps/worker", entry(&[], &["Cargo.toml"])),
        ],
        &[
            ("apps/backend/Cargo.toml", APP_WORKSPACE_CARGO),
            ("apps/backend/apps/worker/Cargo.toml", APP_WORKSPACE_CARGO),
        ],
    ));

    assert!(
        error_results(&results, "RS-ARCH-03").is_empty(),
        "multiple app candidates belong to RS-ARCH-01, not RS-ARCH-03: {results:#?}"
    );
}

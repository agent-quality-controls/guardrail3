use super::super::super::test_support::{
    APP_WORKSPACE_CARGO, PACKAGE_CARGO, assert_error_files, check_results, entry, error_results,
    tree,
};

#[test]
fn nested_app_and_package_zone_roots_hit_exact_overlap_set() {
    let results = check_results(&tree(
        &[
            ("", entry(&["apps", "packages"], &[])),
            ("apps", entry(&["backend"], &[])),
            ("apps/backend", entry(&["packages"], &["Cargo.toml"])),
            ("apps/backend/packages", entry(&["shared"], &[])),
            ("apps/backend/packages/shared", entry(&[], &["Cargo.toml"])),
            ("packages", entry(&["core"], &[])),
            ("packages/core", entry(&["apps"], &["Cargo.toml"])),
            ("packages/core/apps", entry(&["web"], &[])),
            ("packages/core/apps/web", entry(&[], &["Cargo.toml"])),
        ],
        &[
            ("apps/backend/Cargo.toml", APP_WORKSPACE_CARGO),
            ("apps/backend/packages/shared/Cargo.toml", PACKAGE_CARGO),
            ("packages/core/Cargo.toml", PACKAGE_CARGO),
            ("packages/core/apps/web/Cargo.toml", APP_WORKSPACE_CARGO),
        ],
    ));

    assert_error_files(
        &results,
        "RS-ARCH-04",
        &[
            "apps/backend/packages/shared/Cargo.toml",
            "packages/core/Cargo.toml",
        ],
    );
    assert!(
        error_results(&results, "RS-ARCH-04")
            .iter()
            .all(|result| result.severity == guardrail3_domain_report::Severity::Error),
        "RS-ARCH-04 severity drifted: {results:#?}"
    );
}

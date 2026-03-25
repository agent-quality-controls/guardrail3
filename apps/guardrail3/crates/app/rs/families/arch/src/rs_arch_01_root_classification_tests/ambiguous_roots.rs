use super::super::super::test_support::{
    APP_WORKSPACE_CARGO, PACKAGE_CARGO, assert_error_files, check_results, entry, error_results,
    tree,
};

#[test]
fn nested_package_root_inside_app_zone_is_classified_as_ambiguous() {
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

    assert_error_files(
        &results,
        "RS-ARCH-01",
        &["apps/backend/packages/shared/Cargo.toml"],
    );
    assert!(
        error_results(&results, "RS-ARCH-01")
            .iter()
            .all(|result| result.severity == guardrail3_domain_report::Severity::Error),
        "RS-ARCH-01 severity drifted: {results:#?}"
    );
}

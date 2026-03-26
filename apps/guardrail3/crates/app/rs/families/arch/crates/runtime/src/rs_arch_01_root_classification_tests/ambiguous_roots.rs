use guardrail3_app_rs_family_arch_assertions::rs_arch_01_root_classification as assertions;
#[allow(unused_imports)]
use test_support::{APP_WORKSPACE_CARGO, PACKAGE_CARGO, entry, tree, tree_at};

#[test]
fn nested_package_root_inside_app_zone_is_classified_as_ambiguous() {
    let results = assertions::check_results(&tree(
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

    assertions::assert_error_files(
        &results,
        "RS-ARCH-01",
        &["apps/backend/packages/shared/Cargo.toml"],
    );
    assert!(
        assertions::error_results(&results, "RS-ARCH-01")
            .iter()
            .all(|result| result.severity == guardrail3_domain_report::Severity::Error),
        "RS-ARCH-01 severity drifted: {results:#?}"
    );
}

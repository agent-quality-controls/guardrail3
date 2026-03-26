use guardrail3_app_rs_family_arch_assertions::rs_arch_06_owner_family_enablement_coherence as assertions;
#[allow(unused_imports)]
use test_support::{APP_WORKSPACE_CARGO, PACKAGE_CARGO, entry, tree, tree_at};

#[test]
fn golden_layout_has_no_owner_family_coherence_errors() {
    let results = assertions::check_results(&tree(
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
        assertions::error_results(&results, "RS-ARCH-06").is_empty(),
        "unexpected owner-family coherence errors: {results:#?}"
    );
}

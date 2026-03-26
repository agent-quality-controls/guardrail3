use guardrail3_app_rs_family_arch_assertions::rs_arch_06_owner_family_enablement_coherence as assertions;
#[allow(unused_imports)]
use test_support::{APP_WORKSPACE_CARGO, PACKAGE_CARGO, entry, tree, tree_at};

#[test]
fn app_roots_error_when_effective_hexarch_enablement_is_false() {
    let config = "[rust.checks]\narch = true\nhexarch = false\nlibarch = true\n";
    let results = assertions::check_results(&tree(
        &[
            ("", entry(&["apps"], &["guardrail3.toml"])),
            ("apps", entry(&["backend"], &[])),
            ("apps/backend", entry(&[], &["Cargo.toml"])),
        ],
        &[
            ("guardrail3.toml", config),
            ("apps/backend/Cargo.toml", APP_WORKSPACE_CARGO),
        ],
    ));

    assertions::assert_error_files(&results, "RS-ARCH-06", &["apps/backend/Cargo.toml"]);
    assert!(
        assertions::error_results(&results, "RS-ARCH-06")
            .iter()
            .all(|result| result.severity == guardrail3_domain_report::Severity::Error),
        "RS-ARCH-06 severity drifted: {results:#?}"
    );
}

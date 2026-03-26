use guardrail3_app_rs_family_arch_assertions::rs_arch_06_owner_family_enablement_coherence as assertions;
#[allow(unused_imports)]
use test_support::{APP_WORKSPACE_CARGO, PACKAGE_CARGO, entry, tree, tree_at};

#[test]
fn package_roots_error_when_effective_libarch_enablement_is_false() {
    let config = "[rust.checks]\narch = true\nhexarch = true\nlibarch = false\n";
    let results = assertions::check_results(&tree(
        &[
            ("", entry(&["packages"], &["guardrail3.toml"])),
            ("packages", entry(&["shared"], &[])),
            ("packages/shared", entry(&[], &["Cargo.toml"])),
        ],
        &[
            ("guardrail3.toml", config),
            ("packages/shared/Cargo.toml", PACKAGE_CARGO),
        ],
    ));

    assertions::assert_error_files(&results, "RS-ARCH-06", &["packages/shared/Cargo.toml"]);
}

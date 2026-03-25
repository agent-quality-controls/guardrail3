use super::super::super::test_support::{
    PACKAGE_CARGO, assert_error_files, check_results, entry, tree,
};

#[test]
fn package_roots_error_when_effective_libarch_enablement_is_false() {
    let config = "[rust.checks]\nhexarch = true\nlibarch = true\n\n[rust.packages.checks]\nlibarch = false\n";
    let results = check_results(&tree(
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

    assert_error_files(&results, "RS-ARCH-05", &["packages/shared/Cargo.toml"]);
}

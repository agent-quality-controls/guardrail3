use g3rs_toolchain_filetree_checks_assertions::rs_toolchain_filetree_04_legacy_file::{
    assert_findings, error, warn,
};

use crate::test_support::input;

#[test]
fn warns_when_only_legacy_toolchain_exists() {
    let results = crate::check(&input(None, Some("rust-toolchain")));

    assert_findings(
        &results,
        &[warn(
            "legacy rust-toolchain file present",
            "Migrate `rust-toolchain` to `rust-toolchain.toml` so components can be declared explicitly.",
            "rust-toolchain",
            false,
        )],
    );
}

#[test]
fn errors_when_legacy_and_modern_toolchains_both_exist() {
    let results = crate::check(&input(Some("rust-toolchain.toml"), Some("rust-toolchain")));

    assert_findings(
        &results,
        &[error(
            "both rust-toolchain files present",
            "Remove the legacy `rust-toolchain` file. rustup reads it instead of `rust-toolchain.toml` when both exist, so your modern config is ignored.",
            "rust-toolchain",
            false,
        )],
    );
}

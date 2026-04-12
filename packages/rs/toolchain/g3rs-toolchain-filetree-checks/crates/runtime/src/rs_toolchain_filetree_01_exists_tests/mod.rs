use g3rs_toolchain_filetree_checks_assertions::rs_toolchain_filetree_01_exists::{
    assert_findings, error, info,
};

use crate::test_support::input;

#[test]
fn inventories_when_modern_toolchain_exists() {
    let results = crate::check(&input(Some("rust-toolchain.toml"), None));

    assert_findings(
        &results,
        &[info(
            "rust-toolchain.toml exists",
            "Found rust-toolchain.toml at workspace root.",
            "rust-toolchain.toml",
            true,
        )],
    );
}

#[test]
fn errors_when_modern_toolchain_is_missing() {
    let results = crate::check(&input(None, None));

    assert_findings(
        &results,
        &[error(
            "rust-toolchain.toml missing",
            "Expected rust-toolchain.toml at workspace root. Create a `rust-toolchain.toml` with a `[toolchain]` section containing `channel` and `components`.",
            "rust-toolchain.toml",
            false,
        )],
    );
}

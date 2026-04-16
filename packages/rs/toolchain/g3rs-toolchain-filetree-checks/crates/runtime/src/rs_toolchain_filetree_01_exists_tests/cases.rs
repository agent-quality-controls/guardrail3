use g3rs_toolchain_filetree_checks_assertions::rs_toolchain_filetree_01_exists as assertions;

use super::helpers::run_check;

#[test]
fn inventories_when_modern_toolchain_exists() {
    let results = run_check(Some("rust-toolchain.toml"), None);

    assertions::assert_findings(
        &results,
        &[assertions::info(
            "rust-toolchain.toml exists",
            "Found rust-toolchain.toml at workspace root.",
            "rust-toolchain.toml",
            true,
        )],
    );
}

#[test]
fn errors_when_modern_toolchain_is_missing() {
    let results = run_check(None, None);

    assertions::assert_findings(
        &results,
        &[assertions::error(
            "rust-toolchain.toml missing",
            "Expected rust-toolchain.toml at workspace root. Create a `rust-toolchain.toml` with a `[toolchain]` section containing `channel` and `components`.",
            "rust-toolchain.toml",
            false,
        )],
    );
}

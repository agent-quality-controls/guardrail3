use g3rs_toolchain_filetree_checks_assertions::rs_toolchain_filetree_04_legacy_file as assertions;

use super::helpers::run_check;

#[test]
fn warns_when_only_legacy_toolchain_exists() {
    let results = run_check(None, Some("rust-toolchain"));

    assertions::assert_findings(
        &results,
        &[assertions::warn(
            "legacy rust-toolchain file present",
            "Migrate `rust-toolchain` to `rust-toolchain.toml` so components can be declared explicitly.",
            "rust-toolchain",
            false,
        )],
    );
}

#[test]
fn errors_when_legacy_and_modern_toolchains_both_exist() {
    let results = run_check(Some("rust-toolchain.toml"), Some("rust-toolchain"));

    assertions::assert_findings(
        &results,
        &[assertions::error(
            "both rust-toolchain files present",
            "Remove the legacy `rust-toolchain` file. rustup reads it instead of `rust-toolchain.toml` when both exist, so your modern config is ignored.",
            "rust-toolchain",
            false,
        )],
    );
}

#[test]
fn stays_quiet_when_only_modern_toolchain_exists() {
    let results = run_check(Some("rust-toolchain.toml"), None);

    assertions::assert_findings(&results, &[]);
}

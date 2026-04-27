use g3rs_fmt_config_checks_assertions::nightly_keys_on_stable::rule as assertions;
use test_support::G3RsFmtToolchainState;

use super::helpers::run_check;

#[test]
fn errors_when_toolchain_manifest_is_unreadable() {
    let results = run_check(
        r#"
edition = "2024"
group_imports = "StdExternalCrate"
"#,
        G3RsFmtToolchainState::Unreadable,
    );

    assertions::assert_findings(
        &results,
        &[assertions::error(
            "rust-toolchain.toml unreadable",
            "Nightly-only rustfmt settings require a readable root rust-toolchain.toml.",
            "rust-toolchain.toml",
            false,
        )],
    );
}

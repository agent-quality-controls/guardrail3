use g3rs_fmt_config_checks_assertions::nightly_keys_on_stable::rule as assertions;
use test_support::G3RsFmtToolchainState;

use super::helpers::run_check;

#[test]
fn errors_when_toolchain_manifest_is_missing() {
    let results = run_check(
        r#"
edition = "2024"
group_imports = "StdExternalCrate"
"#,
        G3RsFmtToolchainState::Missing,
    );

    assertions::assert_findings(
        &results,
        &[assertions::error(
            "rust-toolchain.toml missing",
            "Nightly-only rustfmt settings require a root rust-toolchain.toml to verify the channel.",
            "rust-toolchain.toml",
            false,
        )],
    );
}

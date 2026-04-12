use g3rs_fmt_config_checks_assertions::rs_fmt_config_03_nightly_keys_on_stable as assertions;
use g3rs_fmt_config_checks_types::G3RsFmtToolchainState;

use super::helpers::run_check;

#[test]
fn errors_when_toolchain_manifest_cannot_be_parsed() {
    let results = run_check(
        r#"
edition = "2024"
group_imports = "StdExternalCrate"
"#,
        G3RsFmtToolchainState::ParseError,
    );

    assertions::assert_findings(
        &results,
        &[assertions::error(
            "rust-toolchain.toml parse error",
            "Nightly-only rustfmt settings require a parseable root rust-toolchain.toml.",
            "rust-toolchain.toml",
            false,
        )],
    );
}

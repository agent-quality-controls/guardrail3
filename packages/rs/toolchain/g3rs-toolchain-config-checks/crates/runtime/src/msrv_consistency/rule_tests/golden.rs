use g3rs_toolchain_config_checks_assertions::msrv_consistency::rule as assertions;

use super::helpers::run_check;

#[test]
fn inventories_when_pinned_toolchain_satisfies_msrv() {
    let results = run_check(
        r#"
[toolchain]
channel = "1.85.0"
components = ["clippy", "rustfmt"]
"#,
        r#"
[package]
name = "fixture"
version = "0.1.0"
edition = "2024"
rust-version = "1.85"
"#,
    );

    assertions::assert_findings(
        &results,
        &[assertions::info(
            "pinned toolchain satisfies MSRV",
            "Pinned toolchain `1.85.0` is compatible with Cargo rust-version `1.85`.",
            "rust-toolchain.toml",
            true,
        )],
    );
}

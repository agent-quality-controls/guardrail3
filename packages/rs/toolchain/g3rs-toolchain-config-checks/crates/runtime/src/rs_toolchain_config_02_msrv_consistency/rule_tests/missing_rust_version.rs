use g3rs_toolchain_config_checks_assertions::rs_toolchain_config_02_msrv_consistency::rule as assertions;

use super::helpers::run_check;

#[test]
fn inventories_when_cargo_rust_version_is_missing() {
    let results = run_check(
        r#"
[toolchain]
channel = "1.85.0"
"#,
        r#"
[package]
name = "fixture"
version = "0.1.0"
edition = "2024"
"#,
    );

    assertions::assert_findings(
        &results,
        &[assertions::info(
            "Cargo rust-version not declared",
            "No `rust-version` found in Cargo.toml, so MSRV consistency cannot be checked.",
            "Cargo.toml",
            true,
        )],
    );
}

use g3_toolchain_content_checks_assertions::rs_toolchain_03_msrv_consistency as assertions;

use super::helpers::run_check;

#[test]
fn warns_when_pinned_toolchain_is_older_than_msrv() {
    let results = run_check(
        r#"
[toolchain]
channel = "1.84.0"
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
        &[assertions::warn(
            "pinned toolchain is older than MSRV",
            "Pinned toolchain `1.84.0` is older than Cargo rust-version `1.85`. Either update the pinned toolchain to match or exceed the MSRV, or lower `rust-version` in Cargo.toml.",
            "rust-toolchain.toml",
            false,
        )],
    );
}

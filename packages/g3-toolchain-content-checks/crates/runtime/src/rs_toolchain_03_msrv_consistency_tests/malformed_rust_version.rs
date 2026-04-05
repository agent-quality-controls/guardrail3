use g3_toolchain_content_checks_assertions::rs_toolchain_03_msrv_consistency as assertions;

use super::helpers::run_check;

#[test]
fn errors_when_cargo_rust_version_string_is_unparseable() {
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
rust-version = "stable"
"#,
    );

    assertions::assert_findings(
        &results,
        &[assertions::error(
            "Cargo rust-version is unparseable",
            "Cannot compare pinned toolchain against invalid Cargo rust-version `stable`.",
            "Cargo.toml",
            false,
        )],
    );
}

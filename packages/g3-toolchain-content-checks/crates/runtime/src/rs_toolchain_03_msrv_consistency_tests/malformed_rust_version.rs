use g3_toolchain_content_checks_assertions::rs_toolchain_03_msrv_consistency as assertions;
use g3_toolchain_content_checks_types::G3CargoRustVersion;

use super::helpers::run_check;

#[test]
fn errors_when_cargo_rust_version_is_invalid_type() {
    let results = run_check(
        r#"
[toolchain]
channel = "1.85.0"
"#,
        G3CargoRustVersion::InvalidType,
    );

    assertions::assert_findings(
        &results,
        &[assertions::error(
            "Cargo rust-version is invalid",
            "`Cargo.toml` `rust-version` must be a string version.",
            "Cargo.toml",
            false,
        )],
    );
}

#[test]
fn errors_when_cargo_rust_version_string_is_unparseable() {
    let results = run_check(
        r#"
[toolchain]
channel = "1.85.0"
"#,
        G3CargoRustVersion::Version("stable".to_owned()),
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

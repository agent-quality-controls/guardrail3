use g3_toolchain_content_checks_assertions::rs_toolchain_03_msrv_consistency as assertions;
use g3_toolchain_content_checks_types::G3CargoRustVersion;

use super::helpers::run_check;

#[test]
fn warns_when_pinned_toolchain_is_older_than_msrv() {
    let results = run_check(
        r#"
[toolchain]
channel = "1.84.0"
"#,
        G3CargoRustVersion::Version("1.85".to_owned()),
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

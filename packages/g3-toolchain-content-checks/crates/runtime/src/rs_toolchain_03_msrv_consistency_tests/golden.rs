use g3_toolchain_content_checks_assertions::rs_toolchain_03_msrv_consistency as assertions;
use g3_toolchain_content_checks_types::G3CargoRustVersion;

use super::helpers::run_check;

#[test]
fn inventories_when_pinned_toolchain_satisfies_msrv() {
    let results = run_check(
        r#"
[toolchain]
channel = "1.85.0"
components = ["clippy", "rustfmt"]
"#,
        G3CargoRustVersion::Version("1.85".to_owned()),
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

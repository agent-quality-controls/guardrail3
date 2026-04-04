use g3_toolchain_content_checks_assertions::rs_toolchain_03_msrv_consistency as assertions;
use g3_toolchain_content_checks_types::G3CargoRustVersion;

use super::helpers::run_check;

#[test]
fn errors_when_cargo_manifest_is_missing() {
    let results = run_check(
        r#"
[toolchain]
channel = "1.85.0"
"#,
        G3CargoRustVersion::MissingManifest,
    );

    assertions::assert_findings(
        &results,
        &[assertions::error(
            "Cargo.toml missing blocks MSRV check",
            "Cargo.toml is required to compare pinned toolchain against declared MSRV.",
            "Cargo.toml",
            false,
        )],
    );
}

#[test]
fn errors_when_cargo_manifest_cannot_be_parsed() {
    let results = run_check(
        r#"
[toolchain]
channel = "1.85.0"
"#,
        G3CargoRustVersion::ParseError("Cargo.toml content missing from ProjectTree".to_owned()),
    );

    assertions::assert_findings(
        &results,
        &[assertions::error(
            "Cargo.toml parse error blocks MSRV check",
            "Invalid root Cargo.toml: Cargo.toml content missing from ProjectTree",
            "Cargo.toml",
            false,
        )],
    );
}

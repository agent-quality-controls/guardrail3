use g3rs_fmt_config_checks_assertions::rs_fmt_config_04_edition_mismatch as assertions;
use g3rs_fmt_config_checks_types::G3RsFmtCargoState;

use super::helpers::run_check;

#[test]
fn errors_when_cargo_manifest_cannot_be_parsed() {
    let results = run_check(
        r#"
edition = "2024"
"#,
        G3RsFmtCargoState::ParseError,
    );

    assertions::assert_findings(
        &results,
        &[assertions::error(
            "Cargo.toml parse error",
            "rustfmt edition checks require a parseable root Cargo.toml.",
            "Cargo.toml",
            false,
        )],
    );
}

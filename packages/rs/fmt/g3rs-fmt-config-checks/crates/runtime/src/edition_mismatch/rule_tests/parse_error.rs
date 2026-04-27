use g3rs_fmt_config_checks_assertions::edition_mismatch::rule as assertions;
use test_support::G3RsFmtCargoState;

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

use g3rs_fmt_config_checks_assertions::rs_fmt_config_04_edition_mismatch::rule as assertions;
use test_support::G3RsFmtCargoState;

use super::helpers::run_check;

#[test]
fn errors_when_cargo_manifest_is_unreadable() {
    let results = run_check(
        r#"
edition = "2024"
"#,
        G3RsFmtCargoState::Unreadable,
    );

    assertions::assert_findings(
        &results,
        &[assertions::error(
            "Cargo.toml unreadable",
            "rustfmt edition checks require a readable root Cargo.toml.",
            "Cargo.toml",
            false,
        )],
    );
}

use g3rs_fmt_config_checks_assertions::rs_fmt_config_04_edition_mismatch::rule as assertions;
use test_support::G3RsFmtCargoState;

use super::helpers::run_check;

#[test]
fn stays_quiet_when_editions_match() {
    let results = run_check(
        r#"
edition = "2024"
"#,
        G3RsFmtCargoState::Parsed(
            cargo_toml_parser::parse(
                r#"
[workspace.package]
edition = "2024"
"#,
            )
            .expect("cargo fixture should parse"),
        ),
    );

    assertions::assert_no_findings(&results);
}

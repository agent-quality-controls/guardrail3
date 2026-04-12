use g3rs_fmt_config_checks_assertions::rs_fmt_config_04_edition_mismatch as assertions;
use g3rs_fmt_config_checks_types::G3RsFmtCargoState;

use super::helpers::run_check;

#[test]
fn errors_when_cargo_edition_is_missing() {
    let results = run_check(
        r#"
edition = "2024"
"#,
        G3RsFmtCargoState::Parsed(
            cargo_toml_parser::parse(
                r#"
[workspace]
members = ["crates/*"]
"#,
            )
            .expect("cargo fixture should parse"),
        ),
    );

    assertions::assert_findings(
        &results,
        &[assertions::error(
            "Cargo.toml edition missing",
            "rustfmt edition checks require `[workspace.package].edition` or `[package].edition` in Cargo.toml.",
            "Cargo.toml",
            false,
        )],
    );
}

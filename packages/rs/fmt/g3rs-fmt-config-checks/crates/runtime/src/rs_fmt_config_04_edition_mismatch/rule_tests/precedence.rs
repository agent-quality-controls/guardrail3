use g3rs_fmt_config_checks_assertions::rs_fmt_config_04_edition_mismatch::rule as assertions;
use test_support::G3RsFmtCargoState;

use super::helpers::run_check;

#[test]
fn prefers_workspace_package_edition_over_package_edition() {
    let results = run_check(
        r#"
edition = "2021"
"#,
        G3RsFmtCargoState::Parsed(
            cargo_toml_parser::parse(
                r#"
[workspace.package]
edition = "2024"

[package]
name = "demo"
version = "0.1.0"
edition = "2018"
"#,
            )
            .expect("cargo fixture should parse"),
        ),
    );

    assertions::assert_findings(
        &results,
        &[assertions::warn(
            "rustfmt edition differs from Cargo edition",
            "rustfmt edition `2021` differs from Cargo edition `2024`. Update `edition` in rustfmt.toml to `2024`.",
            "rustfmt.toml",
            false,
        )],
    );
}

use g3rs_fmt_config_checks_assertions::rs_fmt_config_04_edition_mismatch::rule as assertions;
use test_support::parsed_cargo;

use super::helpers::run_check;

#[test]
fn warns_when_rustfmt_edition_differs_from_cargo() {
    let results = run_check(
        r#"
edition = "2021"
"#,
        parsed_cargo(
            r#"
[workspace.package]
edition = "2024"
"#,
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

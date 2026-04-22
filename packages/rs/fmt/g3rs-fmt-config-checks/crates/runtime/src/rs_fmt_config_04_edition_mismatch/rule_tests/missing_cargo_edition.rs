use g3rs_fmt_config_checks_assertions::rs_fmt_config_04_edition_mismatch::rule as assertions;
use test_support::parsed_cargo;

use super::helpers::run_check;

#[test]
fn errors_when_cargo_edition_is_missing() {
    let results = run_check(
        r#"
edition = "2024"
"#,
        parsed_cargo(
            r#"
[workspace]
members = ["crates/*"]
"#,
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

use g3rs_fmt_config_checks_assertions::edition_mismatch::rule as assertions;
use test_support::parsed_cargo;

use super::helpers::run_check;

#[test]
fn uses_package_edition_fallback_when_workspace_package_edition_is_absent() {
    let results = run_check(
        r#"
edition = "2021"
"#,
        parsed_cargo(
            r#"
[package]
name = "demo"
version = "0.1.0"
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

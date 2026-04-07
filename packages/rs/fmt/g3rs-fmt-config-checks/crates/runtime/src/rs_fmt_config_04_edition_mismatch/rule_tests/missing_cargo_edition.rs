use g3rs_fmt_config_checks_assertions::rs_fmt_config_04_edition_mismatch as assertions;

use super::helpers::run_check;

#[test]
fn errors_when_cargo_edition_is_missing() {
    let results = run_check(
        r#"
edition = "2024"
"#,
        r#"
[workspace]
members = ["crates/*"]
"#,
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

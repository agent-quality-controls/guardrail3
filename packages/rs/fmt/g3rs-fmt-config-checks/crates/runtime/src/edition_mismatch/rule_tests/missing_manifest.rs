use g3rs_fmt_config_checks_assertions::edition_mismatch::rule as assertions;
use test_support::G3RsFmtCargoState;

use super::helpers::run_check;

#[test]
fn errors_when_cargo_manifest_is_missing() {
    let results = run_check(
        r#"
edition = "2024"
"#,
        G3RsFmtCargoState::Missing,
    );

    assertions::assert_findings(
        &results,
        &[assertions::error(
            "Cargo.toml missing",
            "rustfmt edition checks require a root Cargo.toml with workspace or package edition.",
            "Cargo.toml",
            false,
        )],
    );
}

use g3rs_fmt_config_checks_assertions::rs_fmt_config_01_settings::rule as assertions;
use test_support::G3RsFmtRustfmtConfigState;

use super::helpers::run_check;

#[test]
fn errors_when_rustfmt_toml_is_unreadable() {
    let results = run_check(
        G3RsFmtRustfmtConfigState::Unreadable,
        r#"
[workspace.package]
edition = "2024"
"#,
    );

    assertions::assert_findings(
        &results,
        &[assertions::error(
            "rustfmt config unreadable",
            "rustfmt config exists but could not be read from disk",
            "rustfmt.toml",
            false,
        )],
    );
}

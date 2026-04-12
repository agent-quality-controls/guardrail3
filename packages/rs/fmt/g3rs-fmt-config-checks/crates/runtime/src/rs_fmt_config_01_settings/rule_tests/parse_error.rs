use g3rs_fmt_config_checks_assertions::rs_fmt_config_01_settings as assertions;
use g3rs_fmt_config_checks_types::G3RsFmtRustfmtConfigState;

use super::helpers::run_check;

#[test]
fn errors_when_rustfmt_toml_cannot_be_parsed() {
    let results = run_check(
        G3RsFmtRustfmtConfigState::ParseError,
        r#"
[workspace.package]
edition = "2024"
"#,
    );

    assertions::assert_findings(
        &results,
        &[assertions::error(
            "rustfmt config parse error",
            "rustfmt config exists but could not be parsed as a TOML table",
            "rustfmt.toml",
            false,
        )],
    );
}

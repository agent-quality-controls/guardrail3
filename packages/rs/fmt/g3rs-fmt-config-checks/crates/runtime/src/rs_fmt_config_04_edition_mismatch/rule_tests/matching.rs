use g3rs_fmt_config_checks_assertions::rs_fmt_config_04_edition_mismatch::rule as assertions;
use test_support::parsed_cargo;

use super::helpers::run_check;

#[test]
fn stays_quiet_when_editions_match() {
    let results = run_check(
        r#"
edition = "2024"
"#,
        parsed_cargo(
            r#"
[workspace.package]
edition = "2024"
"#,
        ),
    );

    assertions::assert_no_findings(&results);
}

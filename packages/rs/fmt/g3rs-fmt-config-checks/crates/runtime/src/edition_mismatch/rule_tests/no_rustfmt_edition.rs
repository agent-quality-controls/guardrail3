use g3rs_fmt_config_checks_assertions::edition_mismatch::rule as assertions;
use test_support::parsed_cargo;

use super::helpers::run_check;

#[test]
fn stays_quiet_when_rustfmt_edition_is_absent() {
    let results = run_check(
        r"
max_width = 100
",
        parsed_cargo(
            r#"
[workspace.package]
edition = "2024"
"#,
        ),
    );

    assertions::assert_no_findings(&results);
}

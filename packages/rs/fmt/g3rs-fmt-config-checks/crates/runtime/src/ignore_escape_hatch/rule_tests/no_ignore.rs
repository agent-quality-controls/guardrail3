use g3rs_fmt_config_checks_assertions::ignore_escape_hatch::rule as assertions;

use super::helpers::run_check;

#[test]
fn stays_quiet_when_ignore_is_absent() {
    let results = run_check(
        r#"
edition = "2024"
max_width = 100
"#,
        Vec::new(),
    );

    assertions::assert_no_findings(&results);
}

use g3_fmt_content_checks_assertions::rs_fmt_06_edition_mismatch as assertions;

use super::helpers::run_check;

#[test]
fn stays_quiet_when_editions_match() {
    let results = run_check(
        r#"
edition = "2024"
"#,
        r#"
[workspace.package]
edition = "2024"
"#,
    );

    assertions::assert_no_findings(&results);
}

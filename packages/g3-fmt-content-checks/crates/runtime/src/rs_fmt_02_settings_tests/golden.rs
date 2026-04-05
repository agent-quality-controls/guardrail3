use g3_fmt_content_checks_assertions::rs_fmt_02_settings as assertions;

use super::helpers::run_check;

#[test]
fn emits_no_findings_for_baseline_settings() {
    let results = run_check(
        r#"
edition = "2024"
style_edition = "2024"
max_width = 100
tab_spaces = 4
use_field_init_shorthand = true
use_try_shorthand = true
reorder_imports = true
reorder_modules = true
"#,
        r#"
[workspace.package]
edition = "2024"
"#,
    );

    assertions::assert_no_findings(&results);
}

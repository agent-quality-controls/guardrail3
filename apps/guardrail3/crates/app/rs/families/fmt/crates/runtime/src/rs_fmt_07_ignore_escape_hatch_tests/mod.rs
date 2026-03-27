use guardrail3_app_rs_family_fmt_assertions::rs_fmt_07_ignore_escape_hatch as assertions;

use super::run_check;

#[test]
fn reports_ignore_escape_hatches() {
    let results = run_check(
        toml::from_str::<toml::Value>(
            r#"
edition = "2024"
max_width = 100
tab_spaces = 4
use_field_init_shorthand = true
use_try_shorthand = true
reorder_imports = true
reorder_modules = true
ignore = ["generated/**"]
"#,
        )
        .expect("valid TOML"),
    );

    assertions::assert_ignore_escape_hatch(
        &results,
        "`ignore` excludes paths from formatting: [\"generated/**\"]",
        "rustfmt.toml",
    );
}

#[test]
fn emits_no_result_when_ignore_is_absent() {
    let results = run_check(
        toml::from_str::<toml::Value>(
            r#"
edition = "2024"
max_width = 100
tab_spaces = 4
use_field_init_shorthand = true
use_try_shorthand = true
reorder_imports = true
reorder_modules = true
"#,
        )
        .expect("valid TOML"),
    );

    assertions::assert_no_findings(&results);
}

use guardrail3_app_rs_family_fmt_assertions::rs_fmt_03_extra_settings as assertions;

use super::run_check;

#[test]
fn inventories_extra_nonstandard_root_settings() {
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
newline_style = "Unix"
"#,
        )
        .expect("valid TOML"),
    );

    assertions::assert_extra_setting_inventory(&results, "newline_style", "rustfmt.toml");
}

#[test]
fn does_not_treat_ignore_as_generic_extra_setting() {
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

    assertions::assert_no_findings(&results);
}

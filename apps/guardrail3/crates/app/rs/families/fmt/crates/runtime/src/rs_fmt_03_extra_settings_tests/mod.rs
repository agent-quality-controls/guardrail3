use guardrail3_app_rs_family_fmt_assertions::rs_fmt_03_extra_settings as assertions;

use super::run_check;

fn parse_rustfmt_extra_settings_fixture(source: &str) -> toml::Value {
    toml::from_str::<toml::Value>(source).expect("RS-FMT-03 test fixture rustfmt TOML should parse")
}

#[test]
fn inventories_extra_nonstandard_root_settings() {
    let results = run_check(parse_rustfmt_extra_settings_fixture(
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
    ));

    assertions::assert_extra_setting_inventory(&results, "newline_style", "rustfmt.toml");
}

#[test]
fn does_not_treat_ignore_as_generic_extra_setting() {
    let results = run_check(parse_rustfmt_extra_settings_fixture(
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
    ));

    assertions::assert_no_findings(&results);
}

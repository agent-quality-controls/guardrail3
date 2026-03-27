use guardrail3_app_rs_family_fmt_assertions::rs_fmt_02_settings as assertions;

use super::run_check;

#[test]
fn reports_parse_errors_directly() {
    let results = run_check(None);

    assertions::assert_count(&results, 1);
    assertions::assert_parse_error(&results, "rustfmt.toml");
}

#[test]
fn reports_missing_required_setting_with_exact_branch() {
    let results = run_check(Some(
        toml::from_str::<toml::Value>("edition = \"2024\"").expect("valid TOML"),
    ));

    assertions::assert_count(&results, 6);
    assertions::assert_warn_present(
        &results,
        "rustfmt max_width missing",
        "max_width must be set to 100",
        "rustfmt.toml",
    );
}

#[test]
fn reports_wrong_required_setting_with_exact_branch() {
    let results = run_check(Some(
        toml::from_str::<toml::Value>(
            r#"
edition = "2024"
max_width = 120
tab_spaces = 4
use_field_init_shorthand = true
use_try_shorthand = true
reorder_imports = true
reorder_modules = true
"#,
        )
        .expect("valid TOML"),
    ));

    assertions::assert_count(&results, 1);
    assertions::assert_warn_present(
        &results,
        "rustfmt max_width wrong",
        "max_width = 120 but expected 100",
        "rustfmt.toml",
    );
}

#[test]
fn emits_no_results_when_all_required_settings_match() {
    let results = run_check(Some(
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
    ));

    assertions::assert_no_findings(&results);
}

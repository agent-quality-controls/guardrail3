mod helpers;
use guardrail3_app_rs_family_fmt_assertions::rs_fmt_02_settings as assertions;

use helpers::{run_check, run_family};

fn parse_rustfmt_settings_fixture(source: &str) -> toml::Value {
    toml::from_str::<toml::Value>(source).expect("RS-FMT-02 test fixture rustfmt TOML should parse")
}

#[test]
fn reports_parse_errors_directly() {
    let results = run_check(None);

    assertions::assert_count(&results, 1);
    assertions::assert_parse_error(&results, "rustfmt.toml");
}

#[test]
fn reports_non_table_root_configs_as_parse_errors() {
    let fixture = tempfile::tempdir().expect("fixture setup should create a temporary directory");
    let root = fixture.path();

    std::fs::write(root.join("rustfmt.toml"), "\"2024\"\n")
        .expect("fixture setup should write scalar rustfmt.toml");
    std::fs::write(
        root.join("Cargo.toml"),
        "[workspace]\nmembers = []\nresolver = \"2\"\n\n[workspace.package]\nedition = \"2024\"\n",
    )
    .expect("fixture setup should write Cargo.toml");
    std::fs::write(
        root.join("rust-toolchain.toml"),
        "[toolchain]\nchannel = \"stable\"\n",
    )
    .expect("fixture setup should write rust-toolchain.toml");

    let results = run_family(root);

    assertions::assert_count(&results, 1);
    assertions::assert_parse_error(&results, "rustfmt.toml");
}

#[test]
fn reports_missing_required_setting_with_exact_branch() {
    let results = run_check(Some(parse_rustfmt_settings_fixture("edition = \"2024\"")));

    assertions::assert_count(&results, 7);
    assertions::assert_warn_present(
        &results,
        "rustfmt style_edition missing",
        "style_edition must be set to 2024",
        "rustfmt.toml",
    );
    assertions::assert_warn_present(
        &results,
        "rustfmt max_width missing",
        "max_width must be set to 100",
        "rustfmt.toml",
    );
}

#[test]
fn reports_wrong_required_setting_with_exact_branch() {
    let results = run_check(Some(parse_rustfmt_settings_fixture(
        r#"
edition = "2024"
style_edition = "2024"
max_width = 120
tab_spaces = 4
use_field_init_shorthand = true
use_try_shorthand = true
reorder_imports = true
reorder_modules = true
"#,
    )));

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
    let results = run_check(Some(parse_rustfmt_settings_fixture(
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
    )));

    assertions::assert_no_findings(&results);
}

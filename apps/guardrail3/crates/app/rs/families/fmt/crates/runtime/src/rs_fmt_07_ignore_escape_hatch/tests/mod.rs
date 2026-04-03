mod helpers;
use guardrail3_domain_config::types::EscapeHatchConfig;

use guardrail3_app_rs_family_fmt_assertions::rs_fmt_07_ignore_escape_hatch as assertions;

use helpers::{run_check, run_check_with_escape_hatches};

fn parse_rustfmt_ignore_fixture(source: &str) -> toml::Value {
    toml::from_str::<toml::Value>(source).expect("RS-FMT-07 test fixture rustfmt TOML should parse")
}

#[test]
fn reports_ignore_escape_hatches() {
    let results = run_check_with_escape_hatches(
        parse_rustfmt_ignore_fixture(
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
        ),
        vec![EscapeHatchConfig::new(
            "fmt".to_owned(),
            "rustfmt.toml".to_owned(),
            "ignore".to_owned(),
            "ignore".to_owned(),
            "Generated code rewrites break formatter stability.".to_owned(),
        )],
    );

    assertions::assert_ignore_escape_hatch(
        &results,
        "`rustfmt.toml` excludes paths from formatting with documented reason `Generated code rewrites break formatter stability.`: [\"generated/**\"]",
        "rustfmt.toml",
    );
    assertions::assert_count_warning(
        &results,
        "`rustfmt.toml` has 1 rustfmt ignore escape hatch.",
    );
}

#[test]
fn reports_missing_reason_for_ignore_escape_hatches() {
    let results = run_check(parse_rustfmt_ignore_fixture(
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

    assertions::assert_findings(
        &results,
        &[
            assertions::Finding {
                severity: guardrail3_domain_report::Severity::Error,
                title: "rustfmt ignore missing reason",
                message: "`rustfmt.toml` uses `ignore = [\"generated/**\"]` without a matching escape-hatch reason. Add an escape-hatch entry in guardrail3.toml with family = \"fmt\", file = \"rustfmt.toml\", kind = \"ignore\", and a reason explaining why these paths are excluded.",
                file: Some("rustfmt.toml"),
                inventory: false,
            },
            assertions::Finding {
                severity: guardrail3_domain_report::Severity::Warn,
                title: "rustfmt ignore count",
                message: "`rustfmt.toml` has 1 rustfmt ignore escape hatch.",
                file: Some("rustfmt.toml"),
                inventory: false,
            },
        ],
    );
}

#[test]
fn emits_no_result_when_ignore_is_absent() {
    let results = run_check(parse_rustfmt_ignore_fixture(
        r#"
edition = "2024"
max_width = 100
tab_spaces = 4
use_field_init_shorthand = true
use_try_shorthand = true
reorder_imports = true
reorder_modules = true
"#,
    ));

    assertions::assert_no_findings(&results);
}

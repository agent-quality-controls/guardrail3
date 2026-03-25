use guardrail3_domain_report::Severity;

use super::super::inputs::RustfmtRootInput;
use super::check;

#[test]
fn reports_ignore_escape_hatches() {
    let parsed = toml::from_str::<toml::Value>(
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
    .expect("valid TOML");
    let input = RustfmtRootInput {
        config_rel: Some("rustfmt.toml"),
        parsed: Some(&parsed),
        workspace_edition: Some("2024"),
        toolchain_channel: Some("stable"),
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-FMT-07");
    assert_eq!(result.severity, Severity::Warn);
    assert_eq!(result.title, "rustfmt ignore escape hatch");
    assert_eq!(
        result.message,
        "`ignore` excludes paths from formatting: [\"generated/**\"]"
    );
    assert_eq!(result.file.as_deref(), Some("rustfmt.toml"));
}

#[test]
fn emits_no_result_when_ignore_is_absent() {
    let parsed = toml::from_str::<toml::Value>(
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
    .expect("valid TOML");
    let input = RustfmtRootInput {
        config_rel: Some("rustfmt.toml"),
        parsed: Some(&parsed),
        workspace_edition: Some("2024"),
        toolchain_channel: Some("stable"),
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(results.is_empty());
}

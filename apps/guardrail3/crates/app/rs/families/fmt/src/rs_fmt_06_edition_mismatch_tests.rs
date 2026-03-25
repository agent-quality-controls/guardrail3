use guardrail3_domain_report::Severity;

use super::super::inputs::RustfmtRootInput;
use super::check;

#[test]
fn reports_rustfmt_edition_mismatch() {
    let parsed = toml::from_str::<toml::Value>(
        r#"
edition = "2021"
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

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-FMT-06");
    assert_eq!(result.severity, Severity::Warn);
    assert_eq!(result.title, "rustfmt edition differs from Cargo edition");
    assert_eq!(
        result.message,
        "rustfmt edition `2021` differs from Cargo edition `2024`."
    );
    assert_eq!(result.file.as_deref(), Some("rustfmt.toml"));
}

#[test]
fn emits_no_result_when_editions_match() {
    let parsed = toml::from_str::<toml::Value>("edition = \"2024\"").expect("valid TOML");
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

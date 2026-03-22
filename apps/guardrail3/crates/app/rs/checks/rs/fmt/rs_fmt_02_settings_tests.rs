use crate::domain::report::Severity;

use super::super::inputs::RustfmtRootInput;
use super::check;

#[test]
fn reports_parse_errors_directly() {
    let input = RustfmtRootInput {
        config_rel: Some("rustfmt.toml"),
        parsed: None,
        workspace_edition: Some("2024"),
        toolchain_channel: Some("stable"),
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-FMT-02");
    assert_eq!(result.severity, Severity::Error);
    assert_eq!(result.title, "rustfmt config parse error");
    assert_eq!(
        result.message,
        "rustfmt config exists but could not be parsed as TOML"
    );
    assert_eq!(result.file.as_deref(), Some("rustfmt.toml"));
}

#[test]
fn reports_missing_required_setting_with_exact_branch() {
    let parsed = toml::from_str::<toml::Value>("edition = \"2024\"").expect("valid TOML");
    let input = RustfmtRootInput {
        config_rel: Some("rustfmt.toml"),
        parsed: Some(&parsed),
        workspace_edition: Some("2024"),
        toolchain_channel: Some("stable"),
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 6);
    assert!(results.iter().any(|result| {
        result.id == "RS-FMT-02"
            && result.severity == Severity::Warn
            && result.title == "rustfmt max_width missing"
            && result.message == "max_width must be set to 100"
            && result.file.as_deref() == Some("rustfmt.toml")
    }));
}

#[test]
fn reports_wrong_required_setting_with_exact_branch() {
    let parsed = toml::from_str::<toml::Value>(
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
    assert_eq!(result.id, "RS-FMT-02");
    assert_eq!(result.severity, Severity::Warn);
    assert_eq!(result.title, "rustfmt max_width wrong");
    assert_eq!(result.message, "max_width = 120 but expected 100");
    assert_eq!(result.file.as_deref(), Some("rustfmt.toml"));
}

#[test]
fn emits_no_results_when_all_required_settings_match() {
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

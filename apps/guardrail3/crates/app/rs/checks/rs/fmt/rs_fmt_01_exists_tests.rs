use crate::domain::report::Severity;

use super::super::inputs::RustfmtRootInput;
use super::check;

#[test]
fn inventories_when_root_rustfmt_config_exists() {
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
    assert_eq!(result.id, "RS-FMT-01");
    assert!(result.inventory);
    assert_eq!(result.severity, Severity::Info);
    assert_eq!(result.title, "rustfmt config exists");
    assert_eq!(result.message, "Found rustfmt config at workspace root");
    assert_eq!(result.file.as_deref(), Some("rustfmt.toml"));
}

#[test]
fn errors_when_root_rustfmt_config_is_missing() {
    let input = RustfmtRootInput {
        config_rel: None,
        parsed: None,
        workspace_edition: Some("2024"),
        toolchain_channel: Some("stable"),
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-FMT-01");
    assert!(!result.inventory);
    assert_eq!(result.severity, Severity::Error);
    assert_eq!(result.title, "rustfmt config missing");
    assert_eq!(
        result.message,
        "Expected rustfmt.toml or .rustfmt.toml at workspace root"
    );
    assert_eq!(result.file.as_deref(), Some(""));
}

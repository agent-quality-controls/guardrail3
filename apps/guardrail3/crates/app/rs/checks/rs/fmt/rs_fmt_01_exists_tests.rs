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

    assert!(results.iter().any(|result| {
        result.id == "RS-FMT-01"
            && result.inventory
            && result.severity == Severity::Info
            && result.title == "rustfmt config exists"
            && result.message == "Found rustfmt config at workspace root"
            && result.file.as_deref() == Some("rustfmt.toml")
    }));
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

    assert!(results.iter().any(|result| {
        result.id == "RS-FMT-01"
            && !result.inventory
            && result.severity == Severity::Error
            && result.title == "rustfmt config missing"
            && result.message == "Expected rustfmt.toml or .rustfmt.toml at workspace root"
            && result.file.as_deref() == Some("")
    }));
}

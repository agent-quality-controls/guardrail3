use guardrail3_domain_report::Severity;

use super::super::inputs::ToolchainRootInput;
use super::check;

#[test]
fn inventories_when_toolchain_toml_exists() {
    let input = ToolchainRootInput {
        toolchain_toml_rel: Some("rust-toolchain.toml"),
        legacy_toolchain_rel: None,
        parsed: None,
        parse_error: None,
        cargo_rust_version: Some("1.85"),
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-TOOLCHAIN-01");
    assert!(result.inventory);
    assert_eq!(result.severity, Severity::Info);
    assert_eq!(result.title, "rust-toolchain.toml exists");
    assert_eq!(
        result.message,
        "Found rust-toolchain.toml at workspace root."
    );
    assert_eq!(result.file.as_deref(), Some("rust-toolchain.toml"));
}

#[test]
fn errors_when_no_supported_toolchain_file_exists() {
    let input = ToolchainRootInput {
        toolchain_toml_rel: None,
        legacy_toolchain_rel: None,
        parsed: None,
        parse_error: None,
        cargo_rust_version: Some("1.85"),
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-TOOLCHAIN-01");
    assert!(!result.inventory);
    assert_eq!(result.severity, Severity::Error);
    assert_eq!(result.title, "rust-toolchain.toml missing");
    assert_eq!(
        result.message,
        "Expected rust-toolchain.toml at workspace root."
    );
    assert_eq!(result.file.as_deref(), Some(""));
}

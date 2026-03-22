use crate::domain::report::Severity;

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

    assert!(results.iter().any(|result| {
        result.id == "RS-TOOLCHAIN-01"
            && result.inventory
            && result.severity == Severity::Info
            && result.title == "rust-toolchain.toml exists"
            && result.message == "Found rust-toolchain.toml at workspace root."
            && result.file.as_deref() == Some("rust-toolchain.toml")
    }));
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

    assert!(results.iter().any(|result| {
        result.id == "RS-TOOLCHAIN-01"
            && !result.inventory
            && result.severity == Severity::Error
            && result.title == "rust-toolchain.toml missing"
            && result.message == "Expected rust-toolchain.toml at workspace root."
            && result.file.as_deref() == Some("")
    }));
}

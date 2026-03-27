use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::ToolchainRootInput;

const ID: &str = "RS-TOOLCHAIN-01";

pub fn check(input: &ToolchainRootInput<'_>, results: &mut Vec<CheckResult>) {
    match input.toolchain_toml_rel {
        Some(rel) => results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Info,
                title: "rust-toolchain.toml exists".to_owned(),
                message: "Found rust-toolchain.toml at workspace root.".to_owned(),
                file: Some(rel.to_owned()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        ),
        None => results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: "rust-toolchain.toml missing".to_owned(),
            message: "Expected rust-toolchain.toml at workspace root.".to_owned(),
            file: Some("".to_owned()),
            line: None,
            inventory: false,
        }),
    }
}

#[cfg(test)]
pub(crate) fn test_input<'a>(
    toolchain_toml_rel: Option<&'a str>,
    legacy_toolchain_rel: Option<&'a str>,
    parsed: Option<&'a toml::Value>,
    parse_error: Option<&'a str>,
    cargo_rust_version: Option<&'a str>,
    cargo_parse_error: Option<&'a str>,
) -> ToolchainRootInput<'a> {
    ToolchainRootInput {
        toolchain_toml_rel,
        legacy_toolchain_rel,
        parsed,
        parse_error,
        cargo_toml_rel: Some("Cargo.toml"),
        cargo_rust_version,
        cargo_parse_error,
    }
}

#[cfg(test)]
#[path = "rs_toolchain_01_exists_tests/mod.rs"] // reason: test-only sidecar module wiring
mod rs_toolchain_01_exists_tests;

use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::ToolchainRootInput;

const ID: &str = "RS-TOOLCHAIN-04";

pub fn check(input: &ToolchainRootInput<'_>, results: &mut Vec<CheckResult>) {
    if let Some(legacy_rel) = input.legacy_toolchain_rel {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Warn,
            title: "legacy rust-toolchain file present".to_owned(),
            message: "Migrate `rust-toolchain` to `rust-toolchain.toml` so components can be declared explicitly."
                .to_owned(),
            file: Some(legacy_rel.to_owned()),
            line: None,
            inventory: false,
        });
    }

    if input.legacy_toolchain_rel.is_some() && input.toolchain_toml_rel.is_some() {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Warn,
            title: "both rust-toolchain files present".to_owned(),
            message: "Remove the legacy `rust-toolchain` file to avoid ambiguity.".to_owned(),
            file: Some("rust-toolchain".to_owned()),
            line: None,
            inventory: false,
        });
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
#[path = "rs_toolchain_04_legacy_file_tests/mod.rs"] // reason: test-only sidecar module wiring
mod rs_toolchain_04_legacy_file_tests;

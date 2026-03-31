use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::ToolchainRootInput;

const ID: &str = "RS-TOOLCHAIN-04";

pub fn check(input: &ToolchainRootInput<'_>, results: &mut Vec<CheckResult>) {
    if let Some(legacy_rel) = input.legacy_toolchain_rel {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Warn,
            "legacy rust-toolchain file present".to_owned(),
            "Migrate `rust-toolchain` to `rust-toolchain.toml` so components can be declared explicitly."
                .to_owned(),
            Some(legacy_rel.to_owned()),
            None,
            false,
        ));
    }

    if let (Some(legacy_rel), Some(_modern_rel)) =
        (input.legacy_toolchain_rel, input.toolchain_toml_rel)
    {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            "both rust-toolchain files present".to_owned(),
            "Remove the legacy `rust-toolchain` file. rustup prefers it over `rust-toolchain.toml`, so the modern contract is shadowed.".to_owned(),
            Some(legacy_rel.to_owned()),
            None,
            false,
        ));
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
    test_input_for_root(
        "",
        "Cargo.toml",
        toolchain_toml_rel,
        legacy_toolchain_rel,
        parsed,
        parse_error,
        cargo_rust_version,
        cargo_parse_error,
    )
}

#[cfg(test)]
pub(crate) fn test_input_for_root<'a>(
    rel_dir: &'a str,
    cargo_rel_path: &'a str,
    toolchain_toml_rel: Option<&'a str>,
    legacy_toolchain_rel: Option<&'a str>,
    parsed: Option<&'a toml::Value>,
    parse_error: Option<&'a str>,
    cargo_rust_version: Option<&'a str>,
    cargo_parse_error: Option<&'a str>,
) -> ToolchainRootInput<'a> {
    ToolchainRootInput {
        rel_dir,
        cargo_rel_path,
        cargo_toml_rel: Some(cargo_rel_path),
        toolchain_toml_rel,
        legacy_toolchain_rel,
        parsed,
        parse_error,
        cargo_rust_version,
        cargo_rust_version_invalid: false,
        cargo_parse_error,
    }
}

#[cfg(test)]
#[path = "rs_toolchain_04_legacy_file_tests/mod.rs"] // reason: test-only sidecar module wiring
mod rs_toolchain_04_legacy_file_tests;

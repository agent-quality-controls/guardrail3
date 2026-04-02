mod rule;
pub use rule::{check};

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

mod tests;

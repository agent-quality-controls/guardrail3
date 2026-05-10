#![allow(
    clippy::expect_used,
    clippy::missing_const_for_fn,
    clippy::missing_panics_doc,
    reason = "assertion helpers are reusable panic-based proof sites for test harnesses"
)]

use rust_toolchain_toml_parser_runtime::Value;
use rust_toolchain_toml_parser_runtime::types::RustToolchainToml;
use rust_toolchain_toml_parser_runtime::types::rust_toolchain_toml::ToolchainSection;

#[must_use]
pub fn parse_fixture(input: &str) -> RustToolchainToml {
    rust_toolchain_toml_parser_runtime::parse(input)
        .expect("should parse valid rust-toolchain.toml")
}

#[must_use]
pub fn toolchain(cfg: &RustToolchainToml) -> &ToolchainSection {
    cfg.toolchain.as_ref().expect("toolchain should be present")
}

pub fn assert_toolchain_absent(cfg: &RustToolchainToml) {
    assert!(cfg.toolchain.is_none(), "toolchain should be absent");
}

pub fn assert_top_level_extra_empty(cfg: &RustToolchainToml) {
    assert!(cfg.extra.is_empty(), "top-level extra should be empty");
}

pub fn assert_toolchain_extra_empty(cfg: &RustToolchainToml) {
    assert!(
        toolchain(cfg).extra.is_empty(),
        "toolchain extra should be empty"
    );
}

pub fn assert_toolchain_fields(
    cfg: &RustToolchainToml,
    channel: Option<&str>,
    path: Option<&str>,
    components: &[&str],
    targets: &[&str],
    profile: Option<&str>,
) {
    let toolchain = toolchain(cfg);
    assert_eq!(toolchain.channel.as_deref(), channel, "channel mismatch");
    assert_eq!(toolchain.path.as_deref(), path, "path mismatch");
    let expected_components = components
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>();
    let expected_targets = targets.iter().map(ToString::to_string).collect::<Vec<_>>();
    assert_eq!(
        toolchain.components, expected_components,
        "components mismatch"
    );
    assert_eq!(toolchain.targets, expected_targets, "targets mismatch");
    assert_eq!(toolchain.profile.as_deref(), profile, "profile mismatch");
}

pub fn assert_top_level_integer_extra(cfg: &RustToolchainToml, key: &str, expected: i64) {
    assert_eq!(
        cfg.extra.get(key).and_then(Value::as_integer),
        Some(expected),
        "top-level extra key should be preserved"
    );
}

pub fn assert_toolchain_string_extra(cfg: &RustToolchainToml, key: &str, expected: &str) {
    assert_eq!(
        toolchain(cfg).extra.get(key).and_then(Value::as_str),
        Some(expected),
        "toolchain extra key should be preserved"
    );
}

pub fn assert_parse_error(err: impl std::fmt::Display) {
    let msg = err.to_string();
    assert!(
        msg.contains("invalid rust-toolchain.toml"),
        "expected error message prefix, got: {msg}",
    );
}

/// Parse rust-toolchain TOML content through the runtime parser.
///
/// # Errors
///
/// Returns the parser error when the input is not valid rust-toolchain TOML.
pub fn parse_error(
    input: &str,
) -> Result<RustToolchainToml, rust_toolchain_toml_parser_runtime::Error> {
    rust_toolchain_toml_parser_runtime::parse(input)
}

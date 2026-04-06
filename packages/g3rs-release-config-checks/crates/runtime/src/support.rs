/// Shared helpers for release config checks.
use cargo_toml_parser::{CargoToml, InheritableValue, VecStringOrBool};
use guardrail3_check_types::{G3CheckResult, G3Severity};

// Silence unused-crate-dependencies for parser crates used transitively via types.
use cliff_toml_parser as _;
use release_plz_toml_parser as _;

/// Whether the crate is publishable (not `publish = false` or `publish = []`).
pub(crate) fn is_publishable(cargo: &CargoToml) -> bool {
    cargo
        .package
        .as_ref()
        .and_then(|package| package.publish.as_ref())
        .map_or(true, |publish| match publish {
            InheritableValue::Value(VecStringOrBool::Bool(false)) => false,
            InheritableValue::Value(VecStringOrBool::VecString(registries)) => {
                !registries.is_empty()
            }
            _ => true,
        })
}

/// Whether the crate has a `[lib]` section (is a library).
pub(crate) fn is_library(cargo: &CargoToml) -> bool {
    cargo.lib.is_some()
}

/// Whether the crate has `[[bin]]` entries (is a binary).
pub(crate) fn is_binary(cargo: &CargoToml) -> bool {
    !cargo.bin.is_empty()
}

/// Extract the crate name from `[package].name`, falling back to the rel path.
pub(crate) fn crate_name(cargo: &CargoToml, cargo_rel_path: &str) -> String {
    cargo
        .package
        .as_ref()
        .and_then(|package| package.name.clone())
        .unwrap_or_else(|| cargo_rel_path.to_owned())
}

/// Build an error result.
pub(crate) fn error(
    id: &str,
    title: impl Into<String>,
    message: impl Into<String>,
    file: &str,
) -> G3CheckResult {
    G3CheckResult::new(
        id.to_owned(),
        G3Severity::Error,
        title.into(),
        message.into(),
        Some(file.to_owned()),
        None,
    )
}

/// Build a warn result.
pub(crate) fn warn(
    id: &str,
    title: impl Into<String>,
    message: impl Into<String>,
    file: &str,
) -> G3CheckResult {
    G3CheckResult::new(
        id.to_owned(),
        G3Severity::Warn,
        title.into(),
        message.into(),
        Some(file.to_owned()),
        None,
    )
}

/// Build an info result.
pub(crate) fn info(
    id: &str,
    title: impl Into<String>,
    message: impl Into<String>,
    file: &str,
) -> G3CheckResult {
    G3CheckResult::new(
        id.to_owned(),
        G3Severity::Info,
        title.into(),
        message.into(),
        Some(file.to_owned()),
        None,
    )
    .into_inventory()
}

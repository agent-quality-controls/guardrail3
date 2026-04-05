use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use toml::Value;

/// Parsed representation of a `rust-toolchain.toml` file.
///
/// The file commonly contains a single `[toolchain]` table. That table is
/// typed, while unknown keys are preserved in `extra` maps for forward
/// compatibility.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RustToolchainToml {
    /// The `[toolchain]` section when present.
    pub toolchain: Option<ToolchainSection>,

    /// Unknown top-level keys, preserved for forward compatibility.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

/// Parsed representation of the `[toolchain]` section.
///
/// Known fields are typed and optional so missing keys remain distinguishable
/// from explicit values. Unknown keys are preserved in `extra`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub struct ToolchainSection {
    /// Toolchain channel such as `stable`, `nightly`, or `1.85.0`.
    pub channel: Option<String>,
    /// Explicit local toolchain path.
    pub path: Option<String>,
    /// Requested installed components.
    #[serde(default)]
    pub components: Vec<String>,
    /// Requested cross-compilation targets.
    #[serde(default)]
    pub targets: Vec<String>,
    /// Requested rustup profile.
    pub profile: Option<String>,

    /// Unknown toolchain keys, preserved for forward compatibility.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

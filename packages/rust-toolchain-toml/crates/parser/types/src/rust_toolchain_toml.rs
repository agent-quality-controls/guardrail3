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
    toolchain: Option<ToolchainSection>,

    /// Unknown top-level keys, preserved for forward compatibility.
    #[serde(flatten)]
    extra: BTreeMap<String, Value>,
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
    channel: Option<String>,
    /// Requested installed components.
    #[serde(default)]
    components: Vec<String>,
    /// Requested cross-compilation targets.
    #[serde(default)]
    targets: Vec<String>,
    /// Requested rustup profile.
    profile: Option<String>,

    /// Unknown toolchain keys, preserved for forward compatibility.
    #[serde(flatten)]
    extra: BTreeMap<String, Value>,
}

impl RustToolchainToml {
    /// Return the parsed `[toolchain]` section when present.
    #[must_use]
    pub const fn toolchain(&self) -> Option<&ToolchainSection> {
        self.toolchain.as_ref()
    }

    /// Return unknown top-level keys preserved during parsing.
    #[must_use]
    pub const fn extra(&self) -> &BTreeMap<String, Value> {
        &self.extra
    }
}

impl ToolchainSection {
    /// Return the requested toolchain channel.
    #[must_use]
    pub fn channel(&self) -> Option<&str> {
        self.channel.as_deref()
    }

    /// Return the requested installed components.
    #[must_use]
    pub fn components(&self) -> &[String] {
        &self.components
    }

    /// Return the requested cross-compilation targets.
    #[must_use]
    pub fn targets(&self) -> &[String] {
        &self.targets
    }

    /// Return the requested rustup profile.
    #[must_use]
    pub fn profile(&self) -> Option<&str> {
        self.profile.as_deref()
    }

    /// Return unknown `[toolchain]` keys preserved during parsing.
    #[must_use]
    pub const fn extra(&self) -> &BTreeMap<String, Value> {
        &self.extra
    }
}

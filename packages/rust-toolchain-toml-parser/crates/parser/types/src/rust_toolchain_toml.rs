use std::collections::BTreeMap;

use serde::{Deserialize, Deserializer, Serialize};
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
#[derive(Debug, Clone, PartialEq, Serialize)]
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

#[allow(
    clippy::missing_docs_in_private_items,
    reason = "raw serde intermediary exists only to validate the public toolchain section contract"
)]
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct RawToolchainSection {
    channel: Option<String>,
    path: Option<String>,
    #[serde(default)]
    components: Vec<String>,
    #[serde(default)]
    targets: Vec<String>,
    profile: Option<String>,
    #[serde(flatten)]
    extra: BTreeMap<String, Value>,
}

impl<'de> Deserialize<'de> for ToolchainSection {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = RawToolchainSection::deserialize(deserializer)?;

        if let Some(path) = raw.path.as_deref() {
            if let Some(channel) = raw.channel.as_deref() {
                return Err(serde::de::Error::custom(format!(
                    "cannot specify both channel ({channel}) and path ({path}) simultaneously",
                )));
            }

            let ignored = [
                ("components", !raw.components.is_empty()),
                ("targets", !raw.targets.is_empty()),
                ("profile", raw.profile.is_some()),
            ]
            .into_iter()
            .filter_map(|(name, present)| present.then_some(name))
            .collect::<Vec<_>>();

            if !ignored.is_empty() {
                return Err(serde::de::Error::custom(format!(
                    "toolchain options are ignored for path toolchain ({path}): {}",
                    ignored.join(", "),
                )));
            }
        }

        Ok(Self {
            channel: raw.channel,
            path: raw.path,
            components: raw.components,
            targets: raw.targets,
            profile: raw.profile,
            extra: raw.extra,
        })
    }
}

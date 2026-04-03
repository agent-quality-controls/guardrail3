use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use toml::Value;

/// Dependency ban settings.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct BansConfig {
    /// Action for multiple versions of the same crate.
    pub multiple_versions: Option<String>,
    /// Action for wildcard dependencies.
    pub wildcards: Option<String>,
    /// Whether to allow wildcard versions for path dependencies.
    pub allow_wildcard_paths: Option<bool>,
    /// Highlight mode for duplicate versions: `"all"`, `"lowest-version"`, `"simplest-path"`.
    pub highlight: Option<String>,
    /// Crates to explicitly deny.
    #[serde(default)]
    pub deny: Vec<BanDenyEntry>,
    /// Crates to explicitly allow.
    #[serde(default)]
    pub allow: Vec<BanAllowEntry>,
    /// Specific crate versions to skip in duplicate checks.
    #[serde(default)]
    pub skip: Vec<BanSkipEntry>,
    /// Feature-level ban configuration.
    #[serde(default)]
    pub features: Vec<BanFeatureEntry>,
    /// Additional fields not modeled as typed fields.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

// --- Ban deny entries ---

/// An entry in `[bans].deny`: either a bare crate name string
/// or a detailed table with name, version, wrappers, and reason.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BanDenyEntry {
    /// Bare crate name string, e.g. `"openssl"`.
    Simple(String),
    /// Detailed entry: `{ name = "openssl", wrappers = [], reason = "..." }`.
    Detailed(BanDenyDetail),
}

impl BanDenyEntry {
    /// Returns the crate name regardless of entry format.
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        match self {
            Self::Simple(name) => Some(name),
            Self::Detailed(detail) => detail.name(),
        }
    }

    /// Returns the reason if present.
    #[must_use]
    pub fn reason(&self) -> Option<&str> {
        match self {
            Self::Simple(_) => None,
            Self::Detailed(detail) => detail.reason(),
        }
    }
}

/// Detailed ban deny entry.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct BanDenyDetail {
    /// The crate name.
    name: Option<String>,
    /// Alternative crate identifier (deprecated; prefer `name`).
    #[serde(rename = "crate")]
    crate_name: Option<String>,
    /// Version requirement to match.
    version: Option<String>,
    /// Crates that are allowed to transitively depend on the banned crate.
    #[serde(default)]
    wrappers: Vec<String>,
    /// Why this crate is banned.
    reason: Option<String>,
    /// Additional fields not modeled as typed fields.
    #[serde(flatten)]
    extra: BTreeMap<String, Value>,
}

impl BanDenyDetail {
    /// The crate name, if specified.
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Alternative crate identifier (deprecated; prefer `name`).
    #[must_use]
    pub fn crate_name(&self) -> Option<&str> {
        self.crate_name.as_deref()
    }

    /// Version requirement, if specified.
    #[must_use]
    pub fn version(&self) -> Option<&str> {
        self.version.as_deref()
    }

    /// Wrapper crates allowed to depend on this banned crate.
    #[must_use]
    pub fn wrappers(&self) -> &[String] {
        &self.wrappers
    }

    /// Why this crate is banned, if documented.
    #[must_use]
    pub fn reason(&self) -> Option<&str> {
        self.reason.as_deref()
    }

    /// Additional fields not modeled as typed fields.
    #[must_use]
    pub const fn extra(&self) -> &BTreeMap<String, Value> {
        &self.extra
    }
}

// --- Ban allow entries ---

/// An entry in `[bans].allow`: either a bare crate name string
/// or a detailed table.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BanAllowEntry {
    /// Bare crate name string, e.g. `"serde"`.
    Simple(String),
    /// Detailed entry: `{ name = "serde", version = "1.0" }`.
    Detailed(BanAllowDetail),
}

impl BanAllowEntry {
    /// Returns the crate name regardless of entry format.
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        match self {
            Self::Simple(name) => Some(name),
            Self::Detailed(detail) => detail.name(),
        }
    }
}

/// Detailed ban allow entry.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct BanAllowDetail {
    /// The crate name.
    name: Option<String>,
    /// Version requirement to match.
    version: Option<String>,
    /// Additional fields not modeled as typed fields.
    #[serde(flatten)]
    extra: BTreeMap<String, Value>,
}

impl BanAllowDetail {
    /// The crate name, if specified.
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Version requirement, if specified.
    #[must_use]
    pub fn version(&self) -> Option<&str> {
        self.version.as_deref()
    }

    /// Additional fields not modeled as typed fields.
    #[must_use]
    pub const fn extra(&self) -> &BTreeMap<String, Value> {
        &self.extra
    }
}

// --- Ban skip entries ---

/// An entry in `[bans].skip`: either a bare crate name string
/// or a detailed table with name, version, and reason.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BanSkipEntry {
    /// Bare crate name string, e.g. `"windows-sys"`.
    Simple(String),
    /// Detailed entry: `{ name = "windows-sys", version = "=0.48", reason = "..." }`.
    Detailed(BanSkipDetail),
}

impl BanSkipEntry {
    /// Returns the crate name regardless of entry format.
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        match self {
            Self::Simple(name) => Some(name),
            Self::Detailed(detail) => detail.name(),
        }
    }

    /// Returns the reason if present.
    #[must_use]
    pub fn reason(&self) -> Option<&str> {
        match self {
            Self::Simple(_) => None,
            Self::Detailed(detail) => detail.reason(),
        }
    }
}

/// Detailed ban skip entry.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct BanSkipDetail {
    /// The crate name.
    name: Option<String>,
    /// Version requirement to skip.
    version: Option<String>,
    /// Why this skip is necessary.
    reason: Option<String>,
    /// Additional fields not modeled as typed fields.
    #[serde(flatten)]
    extra: BTreeMap<String, Value>,
}

impl BanSkipDetail {
    /// The crate name, if specified.
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Version requirement, if specified.
    #[must_use]
    pub fn version(&self) -> Option<&str> {
        self.version.as_deref()
    }

    /// Why this skip is necessary, if documented.
    #[must_use]
    pub fn reason(&self) -> Option<&str> {
        self.reason.as_deref()
    }

    /// Additional fields not modeled as typed fields.
    #[must_use]
    pub const fn extra(&self) -> &BTreeMap<String, Value> {
        &self.extra
    }
}

// --- Ban feature entries ---

/// Feature-level ban configuration for a specific crate.
///
/// Appears as `[[bans.features]]` in deny.toml.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct BanFeatureEntry {
    /// The crate name this feature rule applies to.
    name: Option<String>,
    /// Features to deny.
    #[serde(default)]
    deny: Vec<String>,
    /// Features to allow (implicitly denies everything else).
    #[serde(default)]
    allow: Vec<String>,
    /// Whether to treat the feature set as exact.
    exact: Option<bool>,
    /// Additional fields not modeled as typed fields.
    #[serde(flatten)]
    extra: BTreeMap<String, Value>,
}

impl BanFeatureEntry {
    /// The crate name this feature rule applies to, if specified.
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Features to deny.
    #[must_use]
    pub fn deny(&self) -> &[String] {
        &self.deny
    }

    /// Features to allow (implicitly denies everything else).
    #[must_use]
    pub fn allow(&self) -> &[String] {
        &self.allow
    }

    /// Whether to treat the feature set as exact.
    #[must_use]
    pub const fn exact(&self) -> Option<bool> {
        self.exact
    }

    /// Additional fields not modeled as typed fields.
    #[must_use]
    pub const fn extra(&self) -> &BTreeMap<String, Value> {
        &self.extra
    }
}

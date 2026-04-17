use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use toml::Value;

/// License checking settings.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LicensesConfig {
    /// Deprecated version field preserved as parsed data.
    pub version: Option<u32>,
    /// Whether licenses are checked for dev-dependencies.
    pub include_dev: Option<bool>,
    /// Whether licenses are checked for build-dependencies.
    pub include_build: Option<bool>,
    /// Unused allow-list handling.
    pub unused_allowed_license: Option<String>,
    /// Unused exception handling.
    pub unused_license_exception: Option<String>,
    /// Allowed license SPDX identifiers.
    #[serde(default)]
    pub allow: Vec<String>,
    /// Minimum confidence threshold for license detection (0.0 to 1.0).
    pub confidence_threshold: Option<f64>,
    /// Per-crate license exceptions.
    #[serde(default)]
    pub exceptions: Vec<LicenseException>,
    /// Per-crate license clarifications.
    #[serde(default)]
    pub clarify: Vec<LicenseClarification>,
    /// Configuration for private/unpublished crates.
    pub private: Option<LicensesPrivateConfig>,
    /// Additional fields not modeled as typed fields.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

/// A per-crate license exception.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LicenseException {
    /// The crate name.
    pub name: Option<String>,
    /// Alternative crate identifier.
    #[serde(rename = "crate")]
    pub crate_name: Option<String>,
    /// Specific crate version this exception applies to.
    pub version: Option<String>,
    /// Why this exception is needed.
    pub reason: Option<String>,
    /// Allowed licenses for this specific crate.
    #[serde(default)]
    pub allow: Vec<String>,
    /// Additional fields not modeled as typed fields.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

/// Configuration for private/unpublished crates.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LicensesPrivateConfig {
    /// Whether to ignore license requirements for private crates.
    pub ignore: Option<bool>,
    /// Registries whose crates are considered private.
    #[serde(default)]
    pub registries: Vec<String>,
    /// Source registries whose licenses should be ignored.
    #[serde(default)]
    pub ignore_sources: Vec<String>,
    /// Additional fields not modeled as typed fields.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

/// A per-crate license clarification.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LicenseClarification {
    /// Deprecated crate name field.
    pub name: Option<String>,
    /// Crate package spec this clarification applies to.
    #[serde(rename = "crate")]
    pub crate_name: Option<String>,
    /// Deprecated version field for old table format.
    pub version: Option<String>,
    /// SPDX expression to use.
    pub expression: String,
    /// Files that establish the clarification.
    #[serde(default)]
    pub license_files: Vec<LicenseClarificationFile>,
    /// Additional fields not modeled as typed fields.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

/// One license file used to support a clarification.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LicenseClarificationFile {
    /// Crate-relative file path.
    pub path: String,
    /// Opaque file hash from cargo-deny.
    pub hash: u32,
    /// Additional fields not modeled as typed fields.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

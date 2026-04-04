use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use toml::Value;

/// License checking settings.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LicensesConfig {
    /// Allowed license SPDX identifiers.
    #[serde(default)]
    pub allow: Vec<String>,
    /// Minimum confidence threshold for license detection (0.0 to 1.0).
    pub confidence_threshold: Option<f64>,
    /// Per-crate license exceptions.
    #[serde(default)]
    pub exceptions: Vec<LicenseException>,
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
    name: String,
    /// Allowed licenses for this specific crate.
    #[serde(default)]
    allow: Vec<String>,
    /// Additional fields not modeled as typed fields.
    #[serde(flatten)]
    extra: BTreeMap<String, Value>,
}

impl LicenseException {
    /// The crate name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Allowed licenses for this specific crate.
    #[must_use]
    pub fn allow(&self) -> &[String] {
        &self.allow
    }

    /// Additional fields not modeled as typed fields.
    #[must_use]
    pub const fn extra(&self) -> &BTreeMap<String, Value> {
        &self.extra
    }
}

/// Configuration for private/unpublished crates.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LicensesPrivateConfig {
    /// Whether to ignore license requirements for private crates.
    ignore: Option<bool>,
    /// Registries whose crates are considered private.
    #[serde(default)]
    registries: Vec<String>,
    /// Additional fields not modeled as typed fields.
    #[serde(flatten)]
    extra: BTreeMap<String, Value>,
}

impl LicensesPrivateConfig {
    /// Whether to ignore license requirements for private crates.
    #[must_use]
    pub const fn ignore(&self) -> Option<bool> {
        self.ignore
    }

    /// Registries whose crates are considered private.
    #[must_use]
    pub fn registries(&self) -> &[String] {
        &self.registries
    }

    /// Additional fields not modeled as typed fields.
    #[must_use]
    pub const fn extra(&self) -> &BTreeMap<String, Value> {
        &self.extra
    }
}

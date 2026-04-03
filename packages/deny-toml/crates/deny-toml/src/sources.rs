use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use toml::Value;

/// Source restriction settings.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct SourcesConfig {
    /// Action for unknown registries: `"deny"`, `"warn"`, `"allow"`.
    pub unknown_registry: Option<String>,
    /// Action for unknown git sources: `"deny"`, `"warn"`, `"allow"`.
    pub unknown_git: Option<String>,
    /// Allowed registries.
    #[serde(default)]
    pub allow_registry: Vec<String>,
    /// Allowed git sources.
    #[serde(default)]
    pub allow_git: Vec<String>,
    /// Additional fields not modeled as typed fields.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

/// Output formatting configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct OutputConfig {
    /// Feature depth for dependency graph output.
    pub feature_depth: Option<u32>,
    /// Additional fields not modeled as typed fields.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}
